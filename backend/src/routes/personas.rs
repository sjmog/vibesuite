use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json as ResponseJson,
    routing::{get, post, put},
    Json, Router,
};
use serde::Deserialize;
use uuid::Uuid;

use crate::{
    app_state::AppState,
    models::{
        persona::{
            ActionArtifact, CreatePersonaAction, CreatePersonaActivity, CreatePersonaTemplate,
            CreateProjectPersona, PersonaAction, PersonaActionWithArtifacts, PersonaActivity,
            PersonaActivityWithTask, PersonaTemplate, ProjectPersona, ProjectPersonaWithTemplate,
            UpdateProjectPersona, ActivityType, TaskSize,
        },
        ApiResponse,
    },
};

#[derive(Debug, Deserialize)]
pub struct ActionHistoryQuery {
    pub limit: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct ActivityHistoryQuery {
    pub limit: Option<i32>,
    pub activity_type: Option<String>,
}

// Persona Template Routes
pub async fn get_persona_templates(
    State(app_state): State<AppState>,
) -> Result<ResponseJson<ApiResponse<Vec<PersonaTemplate>>>, StatusCode> {
    match PersonaTemplate::find_all(&app_state.db_pool).await {
        Ok(templates) => Ok(ResponseJson(ApiResponse {
            success: true,
            data: Some(templates),
            message: None,
        })),
        Err(e) => {
            tracing::error!("Failed to fetch persona templates: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_persona_template(
    Path(template_id): Path<Uuid>,
    State(app_state): State<AppState>,
) -> Result<ResponseJson<ApiResponse<PersonaTemplate>>, StatusCode> {
    match PersonaTemplate::find_by_id(&app_state.db_pool, template_id).await {
        Ok(Some(template)) => Ok(ResponseJson(ApiResponse {
            success: true,
            data: Some(template),
            message: None,
        })),
        Ok(None) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to fetch persona template {}: {}", template_id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_persona_template(
    State(app_state): State<AppState>,
    Json(data): Json<CreatePersonaTemplate>,
) -> Result<ResponseJson<ApiResponse<PersonaTemplate>>, StatusCode> {
    let template_id = Uuid::new_v4();
    
    // Convert capabilities vector to JSON string
    let capabilities_json = serde_json::to_string(&data.capabilities)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let tool_restrictions_json = serde_json::to_string(&data.tool_restrictions)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let automation_triggers_json = serde_json::to_string(&data.automation_triggers)
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let role_type_val = data.role_type as crate::models::persona::RoleType;

    let result = sqlx::query_as!(
        PersonaTemplate,
        r#"INSERT INTO persona_templates (
            id, name, role_type, default_instructions, description,
            capabilities, tool_restrictions, automation_triggers,
            kudos_quota_daily, is_system
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, FALSE)
        RETURNING 
            id as "id!: Uuid",
            name,
            role_type as "role_type!: crate::models::persona::RoleType",
            default_instructions,
            description,
            capabilities,
            tool_restrictions,
            automation_triggers,
            kudos_quota_daily,
            is_system,
            created_at as "created_at!: chrono::DateTime<chrono::Utc>",
            updated_at as "updated_at!: chrono::DateTime<chrono::Utc>""#,
        template_id,
        data.name,
        role_type_val,
        data.default_instructions,
        data.description,
        capabilities_json,
        tool_restrictions_json,
        automation_triggers_json,
        data.kudos_quota_daily
    )
    .fetch_one(&app_state.db_pool)
    .await;

    match result {
        Ok(template) => Ok(ResponseJson(ApiResponse {
            success: true,
            data: Some(template),
            message: Some("Persona template created successfully".to_string()),
        })),
        Err(e) => {
            tracing::error!("Failed to create persona template: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Project Persona Routes
pub async fn get_project_personas(
    Path(project_id): Path<Uuid>,
    State(app_state): State<AppState>,
) -> Result<ResponseJson<ApiResponse<Vec<ProjectPersonaWithTemplate>>>, StatusCode> {
    match ProjectPersona::find_by_project_id_with_templates(&app_state.db_pool, project_id).await {
        Ok(personas) => Ok(ResponseJson(ApiResponse {
            success: true,
            data: Some(personas),
            message: None,
        })),
        Err(e) => {
            tracing::error!("Failed to fetch personas for project {}: {}", project_id, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_project_persona(
    State(app_state): State<AppState>,
    Json(data): Json<CreateProjectPersona>,
) -> Result<ResponseJson<ApiResponse<ProjectPersona>>, StatusCode> {
    let persona_id = Uuid::new_v4();
    
    match ProjectPersona::create(&app_state.db_pool, &data, persona_id).await {
        Ok(persona) => {
            // Create initial activity for persona creation
            let activity_data = CreatePersonaActivity {
                project_persona_id: persona.id,
                task_id: None,
                activity_type: ActivityType::Imported,
                description: "Persona imported to project".to_string(),
                task_size: TaskSize::Small,
                metadata: Some(serde_json::json!({
                    "import_type": "template_instantiation",
                    "template_id": data.template_id
                }).to_string()),
            };

            if let Err(e) = PersonaActivity::create_with_scoring(&app_state.db_pool, &activity_data, Uuid::new_v4()).await {
                tracing::warn!("Failed to create initial persona activity: {}", e);
            }

            Ok(ResponseJson(ApiResponse {
                success: true,
                data: Some(persona),
                message: Some("Project persona created successfully".to_string()),
            }))
        }
        Err(e) => {
            tracing::error!("Failed to create project persona: {}", e);
            if e.to_string().contains("UNIQUE constraint failed") {
                Err(StatusCode::CONFLICT)
            } else {
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
        }
    }
}

pub async fn update_project_persona(
    Path((project_id, persona_id)): Path<(Uuid, Uuid)>,
    State(app_state): State<AppState>,
    Json(data): Json<UpdateProjectPersona>,
) -> Result<ResponseJson<ApiResponse<ProjectPersona>>, StatusCode> {
    let result = sqlx::query_as!(
        ProjectPersona,
        r#"UPDATE project_personas 
           SET custom_name = COALESCE($1, custom_name),
               custom_instructions = COALESCE($2, custom_instructions),
               is_active = COALESCE($3, is_active),
               updated_at = datetime('now', 'subsec')
           WHERE id = $4 AND project_id = $5
           RETURNING 
               id as "id!: Uuid",
               project_id as "project_id!: Uuid",
               template_id as "template_id!: Uuid",
               custom_name,
               custom_instructions,
               is_active,
               professionalism_score,
               quality_score,
               kudos_quota_used,
               wtf_quota_used,
               last_quota_reset as "last_quota_reset!: chrono::DateTime<chrono::Utc>",
               imported_from_project_id as "imported_from_project_id?: Uuid",
               imported_at as "imported_at?: chrono::DateTime<chrono::Utc>",
               created_at as "created_at!: chrono::DateTime<chrono::Utc>",
               updated_at as "updated_at!: chrono::DateTime<chrono::Utc>""#,
        data.custom_name,
        data.custom_instructions,
        data.is_active,
        persona_id,
        project_id
    )
    .fetch_one(&app_state.db_pool)
    .await;

    match result {
        Ok(persona) => Ok(ResponseJson(ApiResponse {
            success: true,
            data: Some(persona),
            message: Some("Project persona updated successfully".to_string()),
        })),
        Err(sqlx::Error::RowNotFound) => Err(StatusCode::NOT_FOUND),
        Err(e) => {
            tracing::error!("Failed to update project persona: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Persona Activity Routes
pub async fn get_persona_activities(
    Path((_project_id, persona_id)): Path<(Uuid, Uuid)>,
    Query(query): Query<ActivityHistoryQuery>,
    State(app_state): State<AppState>,
) -> Result<ResponseJson<ApiResponse<Vec<PersonaActivityWithTask>>>, StatusCode> {
    let limit = query.limit.unwrap_or(50);
    
    let mut sql = r#"SELECT 
        pa.id as "id!: Uuid",
        pa.project_persona_id as "project_persona_id!: Uuid",
        pa.task_id as "task_id?: Uuid",
        t.title as task_title,
        pa.activity_type as "activity_type!: crate::models::persona::ActivityType",
        pa.description,
        pa.professionalism_change,
        pa.quality_change,
        pa.task_size as "task_size!: crate::models::persona::TaskSize",
        pa.metadata,
        pa.created_at as "created_at!: chrono::DateTime<chrono::Utc>"
    FROM persona_activities pa
    LEFT JOIN tasks t ON pa.task_id = t.id
    WHERE pa.project_persona_id = $1"#.to_string();

    let mut params: Vec<&(dyn sqlx::Encode<sqlx::Sqlite> + Send + Sync)> = vec![&persona_id];
    
    if let Some(activity_type) = &query.activity_type {
        sql.push_str(" AND pa.activity_type = $2");
        params.push(activity_type);
        sql.push_str(" ORDER BY pa.created_at DESC LIMIT $3");
        params.push(&limit);
    } else {
        sql.push_str(" ORDER BY pa.created_at DESC LIMIT $2");
        params.push(&limit);
    }

    // For simplicity, use a basic query since sqlx query! macro doesn't handle dynamic params well
    let result = sqlx::query!(
        r#"SELECT 
            pa.id as "id!: Uuid",
            pa.project_persona_id as "project_persona_id!: Uuid",
            pa.task_id as "task_id?: Uuid",
            t.title as "task_title?",
            pa.activity_type as "activity_type!: crate::models::persona::ActivityType",
            pa.description,
            pa.professionalism_change,
            pa.quality_change,
            pa.task_size as "task_size!: crate::models::persona::TaskSize",
            pa.metadata,
            pa.created_at as "created_at!: chrono::DateTime<chrono::Utc>"
        FROM persona_activities pa
        LEFT JOIN tasks t ON pa.task_id = t.id
        WHERE pa.project_persona_id = $1
        ORDER BY pa.created_at DESC
        LIMIT $2"#,
        persona_id,
        limit
    )
    .fetch_all(&app_state.db_pool)
    .await;

    match result {
        Ok(records) => {
            let activities: Vec<PersonaActivityWithTask> = records
                .into_iter()
                .map(|record| PersonaActivityWithTask {
                    id: record.id,
                    project_persona_id: record.project_persona_id,
                    task_id: record.task_id,
                    task_title: record.task_title,
                    activity_type: record.activity_type,
                    description: record.description,
                    professionalism_change: record.professionalism_change,
                    quality_change: record.quality_change,
                    task_size: record.task_size,
                    metadata: record.metadata,
                    created_at: record.created_at,
                })
                .collect();

            Ok(ResponseJson(ApiResponse {
                success: true,
                data: Some(activities),
                message: None,
            }))
        }
        Err(e) => {
            tracing::error!("Failed to fetch persona activities: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_persona_activity(
    State(app_state): State<AppState>,
    Json(data): Json<CreatePersonaActivity>,
) -> Result<ResponseJson<ApiResponse<PersonaActivity>>, StatusCode> {
    let activity_id = Uuid::new_v4();
    
    match PersonaActivity::create_with_scoring(&app_state.db_pool, &data, activity_id).await {
        Ok(activity) => Ok(ResponseJson(ApiResponse {
            success: true,
            data: Some(activity),
            message: Some("Persona activity created successfully".to_string()),
        })),
        Err(e) => {
            tracing::error!("Failed to create persona activity: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Persona Action Routes
pub async fn get_persona_actions(
    Path((_project_id, persona_id)): Path<(Uuid, Uuid)>,
    Query(query): Query<ActionHistoryQuery>,
    State(app_state): State<AppState>,
) -> Result<ResponseJson<ApiResponse<Vec<PersonaActionWithArtifacts>>>, StatusCode> {
    let limit = query.limit;
    
    match PersonaAction::find_by_persona_id_with_artifacts(&app_state.db_pool, persona_id, limit).await {
        Ok(actions) => Ok(ResponseJson(ApiResponse {
            success: true,
            data: Some(actions),
            message: None,
        })),
        Err(e) => {
            tracing::error!("Failed to fetch persona actions: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_persona_action(
    State(app_state): State<AppState>,
    Json(data): Json<CreatePersonaAction>,
) -> Result<ResponseJson<ApiResponse<PersonaAction>>, StatusCode> {
    let action_id = Uuid::new_v4();
    
    match PersonaAction::create(&app_state.db_pool, &data, action_id).await {
        Ok(action) => Ok(ResponseJson(ApiResponse {
            success: true,
            data: Some(action),
            message: Some("Persona action created successfully".to_string()),
        })),
        Err(e) => {
            tracing::error!("Failed to create persona action: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn create_action_artifact(
    Path(action_id): Path<Uuid>,
    State(app_state): State<AppState>,
    Json(artifact): Json<ActionArtifact>,
) -> Result<ResponseJson<ApiResponse<ActionArtifact>>, StatusCode> {
    let artifact_id = Uuid::new_v4();
    let artifact_type_val = artifact.artifact_type as crate::models::persona::ArtifactType;
    
    let result = sqlx::query_as!(
        ActionArtifact,
        r#"INSERT INTO action_artifacts (
            id, action_id, artifact_type, file_path, content_before,
            content_after, git_hash, output_data, size_bytes
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        RETURNING 
            id as "id!: Uuid",
            action_id as "action_id!: Uuid",
            artifact_type as "artifact_type!: crate::models::persona::ArtifactType",
            file_path,
            content_before,
            content_after,
            git_hash,
            output_data,
            size_bytes,
            created_at as "created_at!: chrono::DateTime<chrono::Utc>""#,
        artifact_id,
        action_id,
        artifact_type_val,
        artifact.file_path,
        artifact.content_before,
        artifact.content_after,
        artifact.git_hash,
        artifact.output_data,
        artifact.size_bytes
    )
    .fetch_one(&app_state.db_pool)
    .await;

    match result {
        Ok(artifact) => Ok(ResponseJson(ApiResponse {
            success: true,
            data: Some(artifact),
            message: Some("Action artifact created successfully".to_string()),
        })),
        Err(e) => {
            tracing::error!("Failed to create action artifact: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

// Bulk import personas for a new project
pub async fn bulk_import_default_personas(
    Path(project_id): Path<Uuid>,
    State(app_state): State<AppState>,
) -> Result<ResponseJson<ApiResponse<Vec<ProjectPersona>>>, StatusCode> {
    // Get all system persona templates
    let templates = match PersonaTemplate::find_all(&app_state.db_pool).await {
        Ok(templates) => templates.into_iter().filter(|t| t.is_system).collect::<Vec<_>>(),
        Err(e) => {
            tracing::error!("Failed to fetch persona templates: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let mut created_personas = Vec::new();
    
    for template in templates {
        let create_data = CreateProjectPersona {
            project_id,
            template_id: template.id,
            custom_name: None,
            custom_instructions: None,
        };
        
        let persona_id = Uuid::new_v4();
        
        match ProjectPersona::create(&app_state.db_pool, &create_data, persona_id).await {
            Ok(persona) => {
                // Create initial activity
                let activity_data = CreatePersonaActivity {
                    project_persona_id: persona.id,
                    task_id: None,
                    activity_type: ActivityType::Imported,
                    description: format!("Default persona {} imported to project", template.name),
                    task_size: TaskSize::Small,
                    metadata: Some(serde_json::json!({
                        "import_type": "bulk_default_import",
                        "template_id": template.id
                    }).to_string()),
                };

                if let Err(e) = PersonaActivity::create_with_scoring(&app_state.db_pool, &activity_data, Uuid::new_v4()).await {
                    tracing::warn!("Failed to create bulk import activity for {}: {}", template.name, e);
                }

                created_personas.push(persona);
            }
            Err(e) => {
                tracing::warn!("Failed to import persona template {}: {}", template.name, e);
            }
        }
    }

    let created_count = created_personas.len();
    Ok(ResponseJson(ApiResponse {
        success: true,
        data: Some(created_personas),
        message: Some(format!("Imported {} default personas to project", created_count)),
    }))
}

pub fn router() -> Router<AppState> {
    Router::new()
        // Persona Template routes
        .route("/templates", get(get_persona_templates).post(create_persona_template))
        .route("/templates/:template_id", get(get_persona_template))
        
        // Project Persona routes
        .route("/projects/:project_id/personas", get(get_project_personas))
        .route("/personas", post(create_project_persona))
        .route("/projects/:project_id/personas/:persona_id", put(update_project_persona))
        
        // Activity routes
        .route("/projects/:project_id/personas/:persona_id/activities", get(get_persona_activities))
        .route("/activities", post(create_persona_activity))
        
        // Action routes
        .route("/projects/:project_id/personas/:persona_id/actions", get(get_persona_actions))
        .route("/actions", post(create_persona_action))
        .route("/actions/:action_id/artifacts", post(create_action_artifact))
        
        // Bulk operations
        .route("/projects/:project_id/personas/import-defaults", post(bulk_import_default_personas))
}