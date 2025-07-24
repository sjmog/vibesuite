use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool, Type};
use ts_rs::TS;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Type, Serialize, Deserialize, PartialEq, TS)]
#[sqlx(type_name = "role_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum RoleType {
    Pm,
    RequirementsEngineer,
    Architect,
    Developer,
    UserRole,
    SystemEngineer,
    DevopsEngineer,
    DatabaseEngineer,
    SecurityEngineer,
    AiEngineer,
    WebDesigner,
    QaEngineer,
    FrontendTester,
    BackendTester,
    Specialist,
}

#[derive(Debug, Clone, Type, Serialize, Deserialize, PartialEq, TS)]
#[sqlx(type_name = "capability_category", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum CapabilityCategory {
    Implementation,
    Testing,
    Architecture,
    Security,
    Design,
    Devops,
    Database,
    AiMl,
    Qa,
    Management,
    Analysis,
}

#[derive(Debug, Clone, Copy, Type, Serialize, Deserialize, PartialEq, TS)]
#[sqlx(type_name = "activity_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum ActivityType {
    TaskAssigned,
    TaskCompleted,
    TaskFailed,
    KudosReceived,
    WtfReceived,
    ProcessViolation,
    QualityIssue,
    Imported,
    ScoreAdjustment,
    Delegation,
    PeerReview,
}

impl std::fmt::Display for ActivityType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActivityType::TaskAssigned => write!(f, "task_assigned"),
            ActivityType::TaskCompleted => write!(f, "task_completed"),
            ActivityType::TaskFailed => write!(f, "task_failed"),
            ActivityType::KudosReceived => write!(f, "kudos_received"),
            ActivityType::WtfReceived => write!(f, "wtf_received"),
            ActivityType::ProcessViolation => write!(f, "process_violation"),
            ActivityType::QualityIssue => write!(f, "quality_issue"),
            ActivityType::Imported => write!(f, "imported"),
            ActivityType::ScoreAdjustment => write!(f, "score_adjustment"),
            ActivityType::Delegation => write!(f, "delegation"),
            ActivityType::PeerReview => write!(f, "peer_review"),
        }
    }
}

#[derive(Debug, Clone, Copy, Type, Serialize, Deserialize, PartialEq, TS)]
#[sqlx(type_name = "action_category", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum ActionCategory {
    FileOperation,
    ToolUsage,
    TaskManagement,
    TeamInteraction,
    ProcessAction,
    GitOperation,
}

#[derive(Debug, Clone, Copy, Type, Serialize, Deserialize, PartialEq, TS)]
#[sqlx(type_name = "action_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum ActionType {
    FileRead,
    FileWrite,
    FileEdit,
    FileDelete,
    BashCommand,
    GitCommit,
    GitBranch,
    GitPr,
    SearchQuery,
    ApiCall,
    TaskAssigned,
    TaskStarted,
    TaskCompleted,
    TaskDelegated,
    KudosGiven,
    WtfIssued,
    PeerReview,
    Collaboration,
    TestsRun,
    BuildExecuted,
}

#[derive(Debug, Clone, Type, Serialize, Deserialize, PartialEq, TS)]
#[sqlx(type_name = "result_status", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum ResultStatus {
    Success,
    Failure,
    Partial,
    Cancelled,
}

#[derive(Debug, Clone, Copy, Type, Serialize, Deserialize, PartialEq, TS)]
#[sqlx(type_name = "artifact_type", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum ArtifactType {
    FileChange,
    CommandOutput,
    GitDiff,
    ApiResponse,
    TestResult,
    BuildArtifact,
}

#[derive(Debug, Clone, Copy, Type, Serialize, Deserialize, PartialEq, TS)]
#[sqlx(type_name = "task_size", rename_all = "snake_case")]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum TaskSize {
    Small,
    Standard,
}

// Capability model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Capability {
    pub id: Uuid,
    pub name: String,
    pub category: CapabilityCategory,
    pub description: String,
    pub keywords: String, // JSON array
    pub created_at: DateTime<Utc>,
}

// Persona Template model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct PersonaTemplate {
    pub id: Uuid,
    pub name: String,
    pub role_type: RoleType,
    pub default_instructions: String,
    pub description: String,
    pub capabilities: String, // JSON array of capability IDs
    pub tool_restrictions: String, // JSON array
    pub automation_triggers: String, // JSON array
    pub kudos_quota_daily: i64, // -1 for unlimited
    pub is_system: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Project Persona model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ProjectPersona {
    pub id: Uuid,
    pub project_id: Uuid,
    pub template_id: Uuid,
    pub custom_name: Option<String>,
    pub custom_instructions: Option<String>,
    pub is_active: bool,
    pub professionalism_score: f64,
    pub quality_score: f64,
    pub kudos_quota_used: i64,
    pub wtf_quota_used: i64,
    pub last_quota_reset: DateTime<Utc>,
    pub imported_from_project_id: Option<Uuid>,
    pub imported_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

// Scoring Rules model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ScoringRule {
    pub id: Uuid,
    pub action_type: String,
    pub task_size: TaskSize,
    pub professionalism_points: f64,
    pub quality_points: f64,
    pub description: String,
}

// Persona Activity model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct PersonaActivity {
    pub id: Uuid,
    pub project_persona_id: Uuid,
    pub task_id: Option<Uuid>,
    pub activity_type: ActivityType,
    pub description: String,
    pub professionalism_change: f64,
    pub quality_change: f64,
    pub task_size: TaskSize,
    pub metadata: Option<String>, // JSON
    pub created_at: DateTime<Utc>,
}

// Persona Action model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct PersonaAction {
    pub id: Uuid,
    pub project_persona_id: Uuid,
    pub task_id: Option<Uuid>,
    pub activity_id: Option<Uuid>,
    pub action_type: ActionType,
    pub action_category: ActionCategory,
    pub tool_name: Option<String>,
    pub parameters: Option<String>, // JSON
    pub result_status: ResultStatus,
    pub execution_time_ms: Option<i64>,
    pub description: String,
    pub created_at: DateTime<Utc>,
}

// Action Artifact model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ActionArtifact {
    pub id: Uuid,
    pub action_id: Uuid,
    pub artifact_type: ArtifactType,
    pub file_path: Option<String>,
    pub content_before: Option<String>,
    pub content_after: Option<String>,
    pub git_hash: Option<String>,
    pub output_data: Option<String>, // JSON
    pub size_bytes: Option<i64>,
    pub created_at: DateTime<Utc>,
}

// Learning Event model
#[derive(Debug, Clone, FromRow, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct LearningEvent {
    pub id: Uuid,
    pub project_persona_id: Option<Uuid>,
    pub event_type: String,
    pub category: String,
    pub insight: String,
    pub relevance_score: f64,
    pub metadata: Option<String>, // JSON
    pub created_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
}

// DTO models for API
#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct CreatePersonaTemplate {
    pub name: String,
    pub role_type: RoleType,
    pub default_instructions: String,
    pub description: String,
    pub capabilities: Vec<String>, // capability names
    pub tool_restrictions: Vec<String>,
    pub automation_triggers: Vec<String>,
    pub kudos_quota_daily: i64,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct CreateProjectPersona {
    pub project_id: Uuid,
    pub template_id: Uuid,
    pub custom_name: Option<String>,
    pub custom_instructions: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct UpdateProjectPersona {
    pub custom_name: Option<String>,
    pub custom_instructions: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct CreatePersonaActivity {
    pub project_persona_id: Uuid,
    pub task_id: Option<Uuid>,
    pub activity_type: ActivityType,
    pub description: String,
    pub task_size: TaskSize,
    pub metadata: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct CreatePersonaAction {
    pub project_persona_id: Uuid,
    pub task_id: Option<Uuid>,
    pub activity_id: Option<Uuid>,
    pub action_type: ActionType,
    pub action_category: ActionCategory,
    pub tool_name: Option<String>,
    pub parameters: Option<String>,
    pub description: String,
}

// Enriched models with joined data
#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ProjectPersonaWithTemplate {
    pub id: Uuid,
    pub project_id: Uuid,
    pub template_id: Uuid,
    pub template_name: String,
    pub template_role_type: RoleType,
    pub template_description: String,
    pub custom_name: Option<String>,
    pub custom_instructions: Option<String>,
    pub is_active: bool,
    pub professionalism_score: f64,
    pub quality_score: f64,
    pub kudos_quota_used: i64,
    pub wtf_quota_used: i64,
    pub last_quota_reset: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct PersonaActivityWithTask {
    pub id: Uuid,
    pub project_persona_id: Uuid,
    pub task_id: Option<Uuid>,
    pub task_title: Option<String>,
    pub activity_type: ActivityType,
    pub description: String,
    pub professionalism_change: f64,
    pub quality_change: f64,
    pub task_size: TaskSize,
    pub metadata: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct PersonaActionWithArtifacts {
    pub id: Uuid,
    pub project_persona_id: Uuid,
    pub task_id: Option<Uuid>,
    pub activity_id: Option<Uuid>,
    pub action_type: ActionType,
    pub action_category: ActionCategory,
    pub tool_name: Option<String>,
    pub parameters: Option<String>,
    pub result_status: ResultStatus,
    pub execution_time_ms: Option<i64>,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub artifacts: Vec<ActionArtifact>,
}

// Implementation methods for database operations
impl PersonaTemplate {
    pub async fn find_all(pool: &SqlitePool) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            PersonaTemplate,
            r#"SELECT 
                id as "id!: Uuid",
                name,
                role_type as "role_type!: RoleType",
                default_instructions,
                description,
                capabilities,
                tool_restrictions,
                automation_triggers,
                kudos_quota_daily,
                is_system,
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            FROM persona_templates
            ORDER BY is_system DESC, name ASC"#
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_by_id(pool: &SqlitePool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            PersonaTemplate,
            r#"SELECT 
                id as "id!: Uuid",
                name,
                role_type as "role_type!: RoleType",
                default_instructions,
                description,
                capabilities,
                tool_restrictions,
                automation_triggers,
                kudos_quota_daily,
                is_system,
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>"
            FROM persona_templates
            WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await
    }
}

impl ProjectPersona {
    pub async fn find_by_project_id_with_templates(
        pool: &SqlitePool,
        project_id: Uuid,
    ) -> Result<Vec<ProjectPersonaWithTemplate>, sqlx::Error> {
        let records = sqlx::query!(
            r#"SELECT 
                pp.id as "id!: Uuid",
                pp.project_id as "project_id!: Uuid",
                pp.template_id as "template_id!: Uuid",
                pt.name as template_name,
                pt.role_type as "template_role_type!: RoleType",
                pt.description as template_description,
                pp.custom_name,
                pp.custom_instructions,
                pp.is_active,
                pp.professionalism_score,
                pp.quality_score,
                pp.kudos_quota_used,
                pp.wtf_quota_used,
                pp.last_quota_reset as "last_quota_reset!: DateTime<Utc>",
                pp.created_at as "created_at!: DateTime<Utc>",
                pp.updated_at as "updated_at!: DateTime<Utc>"
            FROM project_personas pp
            JOIN persona_templates pt ON pp.template_id = pt.id
            WHERE pp.project_id = $1 AND pp.is_active = 1
            ORDER BY pt.name ASC"#,
            project_id
        )
        .fetch_all(pool)
        .await?;

        let personas = records
            .into_iter()
            .map(|record| ProjectPersonaWithTemplate {
                id: record.id,
                project_id: record.project_id,
                template_id: record.template_id,
                template_name: record.template_name,
                template_role_type: record.template_role_type,
                template_description: record.template_description,
                custom_name: record.custom_name,
                custom_instructions: record.custom_instructions,
                is_active: record.is_active,
                professionalism_score: record.professionalism_score,
                quality_score: record.quality_score,
                kudos_quota_used: record.kudos_quota_used,
                wtf_quota_used: record.wtf_quota_used,
                last_quota_reset: record.last_quota_reset,
                created_at: record.created_at,
                updated_at: record.updated_at,
            })
            .collect();

        Ok(personas)
    }

    pub async fn create(
        pool: &SqlitePool,
        data: &CreateProjectPersona,
        persona_id: Uuid,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            ProjectPersona,
            r#"INSERT INTO project_personas (
                id, project_id, template_id, custom_name, custom_instructions,
                is_active, professionalism_score, quality_score,
                kudos_quota_used, wtf_quota_used, last_quota_reset
            ) VALUES ($1, $2, $3, $4, $5, 1, 0.0, 0.0, 0, 0, datetime('now', 'subsec'))
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
                last_quota_reset as "last_quota_reset!: DateTime<Utc>",
                imported_from_project_id as "imported_from_project_id?: Uuid",
                imported_at as "imported_at?: DateTime<Utc>",
                created_at as "created_at!: DateTime<Utc>",
                updated_at as "updated_at!: DateTime<Utc>""#,
            persona_id,
            data.project_id,
            data.template_id,
            data.custom_name,
            data.custom_instructions
        )
        .fetch_one(pool)
        .await
    }
}

impl PersonaActivity {
    pub async fn create_with_scoring(
        pool: &SqlitePool,
        data: &CreatePersonaActivity,
        activity_id: Uuid,
    ) -> Result<Self, sqlx::Error> {
        // Get scoring rules for this activity type and task size
        let activity_type_str = data.activity_type.to_string();
        let task_size_val = data.task_size as TaskSize;
        let scoring_rule = sqlx::query!(
            r#"SELECT professionalism_points, quality_points 
               FROM scoring_rules 
               WHERE action_type = $1 AND task_size = $2"#,
            activity_type_str,
            task_size_val
        )
        .fetch_optional(pool)
        .await?;

        let (p_change, q_change) = scoring_rule
            .map(|rule| (rule.professionalism_points, rule.quality_points))
            .unwrap_or((0.0, 0.0));

        // Create the activity
        let activity_type_val = data.activity_type as ActivityType;
        let task_size_val = data.task_size as TaskSize;
        let activity = sqlx::query_as!(
            PersonaActivity,
            r#"INSERT INTO persona_activities (
                id, project_persona_id, task_id, activity_type, description,
                professionalism_change, quality_change, task_size, metadata
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING 
                id as "id!: Uuid",
                project_persona_id as "project_persona_id!: Uuid",
                task_id as "task_id?: Uuid",
                activity_type as "activity_type!: ActivityType",
                description,
                professionalism_change,
                quality_change,
                task_size as "task_size!: TaskSize",
                metadata,
                created_at as "created_at!: DateTime<Utc>""#,
            activity_id,
            data.project_persona_id,
            data.task_id,
            activity_type_val,
            data.description,
            p_change,
            q_change,
            task_size_val,
            data.metadata
        )
        .fetch_one(pool)
        .await?;

        // Update persona scores
        sqlx::query!(
            r#"UPDATE project_personas 
               SET professionalism_score = professionalism_score + $1,
                   quality_score = quality_score + $2,
                   updated_at = datetime('now', 'subsec')
               WHERE id = $3"#,
            p_change,
            q_change,
            data.project_persona_id
        )
        .execute(pool)
        .await?;

        Ok(activity)
    }
}

impl PersonaAction {
    pub async fn create(
        pool: &SqlitePool,
        data: &CreatePersonaAction,
        action_id: Uuid,
    ) -> Result<Self, sqlx::Error> {
        let action_type_val = data.action_type as ActionType;
        let action_category_val = data.action_category as ActionCategory;
        
        sqlx::query_as!(
            PersonaAction,
            r#"INSERT INTO persona_actions (
                id, project_persona_id, task_id, activity_id, action_type,
                action_category, tool_name, parameters, result_status,
                execution_time_ms, description
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'success', NULL, $9)
            RETURNING 
                id as "id!: Uuid",
                project_persona_id as "project_persona_id!: Uuid",
                task_id as "task_id?: Uuid",
                activity_id as "activity_id?: Uuid",
                action_type as "action_type!: ActionType",
                action_category as "action_category!: ActionCategory",
                tool_name,
                parameters,
                result_status as "result_status!: ResultStatus",
                execution_time_ms,
                description,
                created_at as "created_at!: DateTime<Utc>""#,
            action_id,
            data.project_persona_id,
            data.task_id,
            data.activity_id,
            action_type_val,
            action_category_val,
            data.tool_name,
            data.parameters,
            data.description
        )
        .fetch_one(pool)
        .await
    }

    pub async fn find_by_persona_id_with_artifacts(
        pool: &SqlitePool,
        project_persona_id: Uuid,
        limit: Option<i32>,
    ) -> Result<Vec<PersonaActionWithArtifacts>, sqlx::Error> {
        let limit_value = limit.unwrap_or(100);
        
        let actions = sqlx::query_as!(
            PersonaAction,
            r#"SELECT 
                id as "id!: Uuid",
                project_persona_id as "project_persona_id!: Uuid",
                task_id as "task_id?: Uuid",
                activity_id as "activity_id?: Uuid",
                action_type as "action_type!: ActionType",
                action_category as "action_category!: ActionCategory",
                tool_name,
                parameters,
                result_status as "result_status!: ResultStatus",
                execution_time_ms,
                description,
                created_at as "created_at!: DateTime<Utc>"
            FROM persona_actions
            WHERE project_persona_id = $1
            ORDER BY created_at DESC
            LIMIT $2"#,
            project_persona_id,
            limit_value
        )
        .fetch_all(pool)
        .await?;

        let mut actions_with_artifacts = Vec::new();
        
        for action in actions {
            let artifacts = sqlx::query_as!(
                ActionArtifact,
                r#"SELECT 
                    id as "id!: Uuid",
                    action_id as "action_id!: Uuid",
                    artifact_type as "artifact_type!: ArtifactType",
                    file_path,
                    content_before,
                    content_after,
                    git_hash,
                    output_data,
                    size_bytes,
                    created_at as "created_at!: DateTime<Utc>"
                FROM action_artifacts
                WHERE action_id = $1
                ORDER BY created_at ASC"#,
                action.id
            )
            .fetch_all(pool)
            .await?;

            actions_with_artifacts.push(PersonaActionWithArtifacts {
                id: action.id,
                project_persona_id: action.project_persona_id,
                task_id: action.task_id,
                activity_id: action.activity_id,
                action_type: action.action_type,
                action_category: action.action_category,
                tool_name: action.tool_name,
                parameters: action.parameters,
                result_status: action.result_status,
                execution_time_ms: action.execution_time_ms,
                description: action.description,
                created_at: action.created_at,
                artifacts,
            });
        }

        Ok(actions_with_artifacts)
    }
}