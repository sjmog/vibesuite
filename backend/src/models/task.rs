use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, SqlitePool, Type};
use ts_rs::TS;
use uuid::Uuid;

#[derive(Debug, Clone, Type, Serialize, Deserialize, PartialEq, TS)]
#[sqlx(type_name = "task_status", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
#[ts(export)]
pub enum TaskStatus {
    Todo,
    InProgress,
    InReview,
    Done,
    Cancelled,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct Task {
    pub id: Uuid,
    pub project_id: Uuid, // Foreign key to Project
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub parent_task_attempt: Option<Uuid>, // Foreign key to parent TaskAttempt
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub assigned_persona_id: Option<Uuid>, // Foreign key to ProjectPersona
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct TaskWithAttemptStatus {
    pub id: Uuid,
    pub project_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub parent_task_attempt: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub assigned_persona_id: Option<Uuid>,
    pub has_in_progress_attempt: bool,
    pub has_merged_attempt: bool,
    pub last_attempt_failed: bool,
    pub latest_attempt_executor: Option<String>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct CreateTask {
    pub project_id: Uuid,
    pub title: String,
    pub description: Option<String>,
<<<<<<< HEAD
    pub assigned_persona_id: Option<Uuid>,
=======
    pub parent_task_attempt: Option<Uuid>,
>>>>>>> upstream/main
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct CreateTaskAndStart {
    pub project_id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub parent_task_attempt: Option<Uuid>,
    pub executor: Option<crate::executor::ExecutorConfig>,
}

#[derive(Debug, Deserialize, TS)]
#[ts(export)]
pub struct UpdateTask {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TaskStatus>,
<<<<<<< HEAD
    pub assigned_persona_id: Option<Option<Uuid>>,
=======
    pub parent_task_attempt: Option<Uuid>,
>>>>>>> upstream/main
}

impl Task {
    pub async fn find_by_project_id_with_attempt_status(
        pool: &SqlitePool,
        project_id: Uuid,
    ) -> Result<Vec<TaskWithAttemptStatus>, sqlx::Error> {
        let records = sqlx::query!(
<<<<<<< HEAD
            r#"SELECT 
                t.id                  AS "id!: Uuid", 
                t.project_id          AS "project_id!: Uuid", 
                t.title, 
                t.description, 
                t.status              AS "status!: TaskStatus", 
                t.created_at          AS "created_at!: DateTime<Utc>", 
                t.updated_at          AS "updated_at!: DateTime<Utc>",
                t.assigned_persona_id AS "assigned_persona_id?: Uuid",
                CASE 
                WHEN in_progress_attempts.task_id IS NOT NULL THEN true 
                ELSE false 
                END                   AS "has_in_progress_attempt!: i64",
                CASE 
                WHEN merged_attempts.task_id IS NOT NULL THEN true 
                ELSE false 
                END                   AS "has_merged_attempt!",
                CASE 
                WHEN failed_attempts.task_id IS NOT NULL THEN true 
                ELSE false 
                END                   AS "has_failed_attempt!"
            FROM tasks t
            LEFT JOIN (
                SELECT DISTINCT ta.task_id
                FROM task_attempts ta
                JOIN execution_processes ep 
                ON ta.id = ep.task_attempt_id
                JOIN (
                    -- pick exactly one “latest” activity per process,
                    -- tiebreaking so that running‐states are lower priority
                    SELECT execution_process_id, status
                    FROM (
                        SELECT
                            execution_process_id,
                            status,
                            ROW_NUMBER() OVER (
                                PARTITION BY execution_process_id
                                ORDER BY
                                    created_at DESC,
                                    CASE 
                                    WHEN status IN ('setuprunning','executorrunning') THEN 1 
                                    ELSE 0 
                                    END
                            ) AS rn
                        FROM task_attempt_activities
                    ) sub
                    WHERE rn = 1
                ) latest_act 
                ON ep.id = latest_act.execution_process_id
                WHERE latest_act.status IN ('setuprunning','executorrunning')
            ) in_progress_attempts 
            ON t.id = in_progress_attempts.task_id
            LEFT JOIN (
                SELECT DISTINCT ta.task_id
                FROM task_attempts ta
                WHERE ta.merge_commit IS NOT NULL
            ) merged_attempts 
            ON t.id = merged_attempts.task_id
            LEFT JOIN (
                SELECT DISTINCT latest_attempts.task_id
                FROM (
                    -- Get the latest attempt for each task
                    SELECT task_id, id as attempt_id, created_at,
                           ROW_NUMBER() OVER (PARTITION BY task_id ORDER BY created_at DESC) AS rn
                    FROM task_attempts
                    WHERE merge_commit IS NULL  -- Don't show as failed if already merged
                ) latest_attempts
                JOIN execution_processes ep 
                ON latest_attempts.attempt_id = ep.task_attempt_id
                JOIN (
                    -- pick exactly one "latest" activity per process,
                    -- tiebreaking so that running‐states are lower priority
                    SELECT execution_process_id, status
                    FROM (
                        SELECT
                            execution_process_id,
                            status,
                            ROW_NUMBER() OVER (
                                PARTITION BY execution_process_id
                                ORDER BY
                                    created_at DESC,
                                    CASE 
                                    WHEN status IN ('setuprunning','executorrunning') THEN 1 
                                    ELSE 0 
                                    END
                            ) AS rn
                        FROM task_attempt_activities
                    ) sub
                    WHERE rn = 1
                ) latest_act 
                ON ep.id = latest_act.execution_process_id
                WHERE latest_attempts.rn = 1  -- Only consider the latest attempt
                  AND latest_act.status IN ('setupfailed','executorfailed')
            ) failed_attempts 
            ON t.id = failed_attempts.task_id
            WHERE t.project_id = $1
            ORDER BY t.created_at DESC;
            "#,
=======
            r#"SELECT
  t.id                            AS "id!: Uuid",
  t.project_id                    AS "project_id!: Uuid",
  t.title,
  t.description,
  t.status                        AS "status!: TaskStatus",
  t.parent_task_attempt           AS "parent_task_attempt: Uuid",
  t.created_at                    AS "created_at!: DateTime<Utc>",
  t.updated_at                    AS "updated_at!: DateTime<Utc>",

  CASE WHEN EXISTS (
    SELECT 1
      FROM task_attempts ta
      JOIN execution_processes ep
        ON ep.task_attempt_id = ta.id
     WHERE ta.task_id       = t.id
       AND ep.status        = 'running'
       AND ep.process_type IN ('setupscript','cleanupscript','codingagent')
     LIMIT 1
  ) THEN 1 ELSE 0 END            AS "has_in_progress_attempt!: i64",

  CASE WHEN EXISTS (
    SELECT 1
      FROM task_attempts ta
     WHERE ta.task_id       = t.id
       AND ta.merge_commit IS NOT NULL
     LIMIT 1
  ) THEN 1 ELSE 0 END            AS "has_merged_attempt!: i64",

  CASE WHEN (
    SELECT ep.status
      FROM task_attempts ta
      JOIN execution_processes ep
        ON ep.task_attempt_id = ta.id
     WHERE ta.task_id       = t.id
     AND ep.process_type IN ('setupscript','cleanupscript','codingagent')
     ORDER BY ep.created_at DESC
     LIMIT 1
  ) IN ('failed','killed') THEN 1 ELSE 0 END
                                 AS "last_attempt_failed!: i64",

  ( SELECT ta.executor
      FROM task_attempts ta
     WHERE ta.task_id = t.id
     ORDER BY ta.created_at DESC
     LIMIT 1
  )                               AS "latest_attempt_executor"

FROM tasks t
WHERE t.project_id = $1
ORDER BY t.created_at DESC"#,
>>>>>>> upstream/main
            project_id
        )
        .fetch_all(pool)
        .await?;

        let tasks = records
            .into_iter()
<<<<<<< HEAD
            .map(|record| TaskWithAttemptStatus {
                id: record.id,
                project_id: record.project_id,
                title: record.title,
                description: record.description,
                status: record.status,
                created_at: record.created_at,
                updated_at: record.updated_at,
                assigned_persona_id: record.assigned_persona_id,
                has_in_progress_attempt: record.has_in_progress_attempt != 0,
                has_merged_attempt: record.has_merged_attempt != 0,
                has_failed_attempt: record.has_failed_attempt != 0,
=======
            .map(|rec| TaskWithAttemptStatus {
                id: rec.id,
                project_id: rec.project_id,
                title: rec.title,
                description: rec.description,
                status: rec.status,
                parent_task_attempt: rec.parent_task_attempt,
                created_at: rec.created_at,
                updated_at: rec.updated_at,
                has_in_progress_attempt: rec.has_in_progress_attempt != 0,
                has_merged_attempt: rec.has_merged_attempt != 0,
                last_attempt_failed: rec.last_attempt_failed != 0,
                latest_attempt_executor: rec.latest_attempt_executor,
>>>>>>> upstream/main
            })
            .collect();

        Ok(tasks)
    }

    pub async fn find_by_id(pool: &SqlitePool, id: Uuid) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Task,
<<<<<<< HEAD
            r#"SELECT id as "id!: Uuid", project_id as "project_id!: Uuid", title, description, status as "status!: TaskStatus", created_at as "created_at!: DateTime<Utc>", updated_at as "updated_at!: DateTime<Utc>", assigned_persona_id as "assigned_persona_id?: Uuid"
=======
            r#"SELECT id as "id!: Uuid", project_id as "project_id!: Uuid", title, description, status as "status!: TaskStatus", parent_task_attempt as "parent_task_attempt: Uuid", created_at as "created_at!: DateTime<Utc>", updated_at as "updated_at!: DateTime<Utc>"
>>>>>>> upstream/main
               FROM tasks 
               WHERE id = $1"#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn find_by_id_and_project_id(
        pool: &SqlitePool,
        id: Uuid,
        project_id: Uuid,
    ) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Task,
<<<<<<< HEAD
            r#"SELECT id as "id!: Uuid", project_id as "project_id!: Uuid", title, description, status as "status!: TaskStatus", created_at as "created_at!: DateTime<Utc>", updated_at as "updated_at!: DateTime<Utc>", assigned_persona_id as "assigned_persona_id?: Uuid"
=======
            r#"SELECT id as "id!: Uuid", project_id as "project_id!: Uuid", title, description, status as "status!: TaskStatus", parent_task_attempt as "parent_task_attempt: Uuid", created_at as "created_at!: DateTime<Utc>", updated_at as "updated_at!: DateTime<Utc>"
>>>>>>> upstream/main
               FROM tasks 
               WHERE id = $1 AND project_id = $2"#,
            id,
            project_id
        )
        .fetch_optional(pool)
        .await
    }

    pub async fn create(
        pool: &SqlitePool,
        data: &CreateTask,
        task_id: Uuid,
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Task,
<<<<<<< HEAD
            r#"INSERT INTO tasks (id, project_id, title, description, status, assigned_persona_id) 
               VALUES ($1, $2, $3, $4, $5, $6) 
               RETURNING id as "id!: Uuid", project_id as "project_id!: Uuid", title, description, status as "status!: TaskStatus", created_at as "created_at!: DateTime<Utc>", updated_at as "updated_at!: DateTime<Utc>", assigned_persona_id as "assigned_persona_id?: Uuid""#,
=======
            r#"INSERT INTO tasks (id, project_id, title, description, status, parent_task_attempt) 
               VALUES ($1, $2, $3, $4, $5, $6) 
               RETURNING id as "id!: Uuid", project_id as "project_id!: Uuid", title, description, status as "status!: TaskStatus", parent_task_attempt as "parent_task_attempt: Uuid", created_at as "created_at!: DateTime<Utc>", updated_at as "updated_at!: DateTime<Utc>""#,
>>>>>>> upstream/main
            task_id,
            data.project_id,
            data.title,
            data.description,
            TaskStatus::Todo as TaskStatus,
<<<<<<< HEAD
            data.assigned_persona_id
=======
            data.parent_task_attempt
>>>>>>> upstream/main
        )
        .fetch_one(pool)
        .await
    }

    pub async fn update(
        pool: &SqlitePool,
        id: Uuid,
        project_id: Uuid,
        title: String,
        description: Option<String>,
        status: TaskStatus,
        parent_task_attempt: Option<Uuid>,
    ) -> Result<Self, sqlx::Error> {
        let status_value = status as TaskStatus;
        sqlx::query_as!(
            Task,
            r#"UPDATE tasks 
               SET title = $3, description = $4, status = $5, parent_task_attempt = $6 
               WHERE id = $1 AND project_id = $2 
<<<<<<< HEAD
               RETURNING id as "id!: Uuid", project_id as "project_id!: Uuid", title, description, status as "status!: TaskStatus", created_at as "created_at!: DateTime<Utc>", updated_at as "updated_at!: DateTime<Utc>", assigned_persona_id as "assigned_persona_id?: Uuid""#,
=======
               RETURNING id as "id!: Uuid", project_id as "project_id!: Uuid", title, description, status as "status!: TaskStatus", parent_task_attempt as "parent_task_attempt: Uuid", created_at as "created_at!: DateTime<Utc>", updated_at as "updated_at!: DateTime<Utc>""#,
>>>>>>> upstream/main
            id,
            project_id,
            title,
            description,
            status_value,
            parent_task_attempt
        )
        .fetch_one(pool)
        .await
    }

    pub async fn update_with_persona(
        pool: &SqlitePool,
        id: Uuid,
        project_id: Uuid,
        title: String,
        description: Option<String>,
        status: TaskStatus,
        assigned_persona_id: Option<Uuid>,
    ) -> Result<Self, sqlx::Error> {
        let status_value = status as TaskStatus;
        sqlx::query_as!(
            Task,
            r#"UPDATE tasks 
               SET title = $3, description = $4, status = $5, assigned_persona_id = $6 
               WHERE id = $1 AND project_id = $2 
               RETURNING id as "id!: Uuid", project_id as "project_id!: Uuid", title, description, status as "status!: TaskStatus", created_at as "created_at!: DateTime<Utc>", updated_at as "updated_at!: DateTime<Utc>", assigned_persona_id as "assigned_persona_id?: Uuid""#,
            id,
            project_id,
            title,
            description,
            status_value,
            assigned_persona_id
        )
        .fetch_one(pool)
        .await
    }

    pub async fn update_status(
        pool: &SqlitePool,
        id: Uuid,
        project_id: Uuid,
        status: TaskStatus,
    ) -> Result<(), sqlx::Error> {
        let status_value = status as TaskStatus;
        sqlx::query!(
            "UPDATE tasks SET status = $3, updated_at = CURRENT_TIMESTAMP WHERE id = $1 AND project_id = $2",
            id,
            project_id,
            status_value
        )
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn delete(pool: &SqlitePool, id: Uuid, project_id: Uuid) -> Result<u64, sqlx::Error> {
        let result = sqlx::query!(
            "DELETE FROM tasks WHERE id = $1 AND project_id = $2",
            id,
            project_id
        )
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn exists(
        pool: &SqlitePool,
        id: Uuid,
        project_id: Uuid,
    ) -> Result<bool, sqlx::Error> {
        let result = sqlx::query!(
            "SELECT id as \"id!: Uuid\" FROM tasks WHERE id = $1 AND project_id = $2",
            id,
            project_id
        )
        .fetch_optional(pool)
        .await?;
        Ok(result.is_some())
    }

    pub async fn find_related_tasks_by_attempt_id(
        pool: &SqlitePool,
        attempt_id: Uuid,
        project_id: Uuid,
    ) -> Result<Vec<Self>, sqlx::Error> {
        // Find both children and parent for this attempt
        sqlx::query_as!(
            Task,
            r#"SELECT DISTINCT t.id as "id!: Uuid", t.project_id as "project_id!: Uuid", t.title, t.description, t.status as "status!: TaskStatus", t.parent_task_attempt as "parent_task_attempt: Uuid", t.created_at as "created_at!: DateTime<Utc>", t.updated_at as "updated_at!: DateTime<Utc>"
               FROM tasks t
               WHERE (
                   -- Find children: tasks that have this attempt as parent
                   t.parent_task_attempt = $1 AND t.project_id = $2
               ) OR (
                   -- Find parent: task that owns the parent attempt of current task
                   EXISTS (
                       SELECT 1 FROM tasks current_task 
                       JOIN task_attempts parent_attempt ON current_task.parent_task_attempt = parent_attempt.id
                       WHERE parent_attempt.task_id = t.id 
                       AND parent_attempt.id = $1 
                       AND current_task.project_id = $2
                   )
               )
               -- Exclude the current task itself to prevent circular references
               AND t.id != (SELECT task_id FROM task_attempts WHERE id = $1)
               ORDER BY t.created_at DESC"#,
            attempt_id,
            project_id
        )
        .fetch_all(pool)
        .await
    }
}
