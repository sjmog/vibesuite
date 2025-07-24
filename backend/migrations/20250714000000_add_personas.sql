PRAGMA foreign_keys = ON;

-- Create capabilities table for modular capability system
CREATE TABLE capabilities (
    id              BLOB PRIMARY KEY,
    name            TEXT NOT NULL UNIQUE,
    category        TEXT NOT NULL,
    description     TEXT NOT NULL DEFAULT '',
    keywords        TEXT NOT NULL DEFAULT '', -- JSON array of matching keywords
    created_at      TEXT NOT NULL DEFAULT (datetime('now', 'subsec')),
    CHECK (category IN ('implementation', 'testing', 'architecture', 'security', 'design', 'devops', 'database', 'ai_ml', 'qa', 'management', 'analysis'))
);

-- Create persona templates (global, reusable across projects)
CREATE TABLE persona_templates (
    id                   BLOB PRIMARY KEY,
    name                 TEXT NOT NULL UNIQUE,
    role_type            TEXT NOT NULL,
    default_instructions TEXT NOT NULL DEFAULT '',
    description          TEXT NOT NULL DEFAULT '',
    capabilities         TEXT NOT NULL DEFAULT '', -- JSON array of capability IDs
    tool_restrictions    TEXT NOT NULL DEFAULT '', -- JSON array of restricted tools
    automation_triggers  TEXT NOT NULL DEFAULT '', -- JSON array of auto-triggers
    kudos_quota_daily    INTEGER NOT NULL DEFAULT 1, -- PM: unlimited (-1), Architect: 3, others: 1
    is_system            BOOLEAN NOT NULL DEFAULT FALSE, -- true for built-in templates
    created_at           TEXT NOT NULL DEFAULT (datetime('now', 'subsec')),
    updated_at           TEXT NOT NULL DEFAULT (datetime('now', 'subsec')),
    CHECK (role_type IN ('pm', 'requirements_engineer', 'architect', 'developer', 'user_role', 'system_engineer', 'devops_engineer', 'database_engineer', 'security_engineer', 'ai_engineer', 'web_designer', 'qa_engineer', 'frontend_tester', 'backend_tester', 'specialist'))
);

-- Create project-specific persona instances
CREATE TABLE project_personas (
    id                    BLOB PRIMARY KEY,
    project_id            BLOB NOT NULL,
    template_id           BLOB NOT NULL,
    custom_name           TEXT, -- override template name if needed
    custom_instructions   TEXT, -- override/extend template instructions
    is_active             BOOLEAN NOT NULL DEFAULT TRUE,
    professionalism_score REAL NOT NULL DEFAULT 0.0,
    quality_score         REAL NOT NULL DEFAULT 0.0,
    kudos_quota_used      INTEGER NOT NULL DEFAULT 0,
    wtf_quota_used        INTEGER NOT NULL DEFAULT 0,
    last_quota_reset      TEXT NOT NULL DEFAULT (date('now')),
    imported_from_project_id BLOB, -- track if imported from another project
    imported_at           TEXT,
    created_at            TEXT NOT NULL DEFAULT (datetime('now', 'subsec')),
    updated_at            TEXT NOT NULL DEFAULT (datetime('now', 'subsec')),
    FOREIGN KEY (project_id) REFERENCES projects(id) ON DELETE CASCADE,
    FOREIGN KEY (template_id) REFERENCES persona_templates(id) ON DELETE RESTRICT,
    FOREIGN KEY (imported_from_project_id) REFERENCES projects(id) ON DELETE SET NULL,
    UNIQUE(project_id, template_id) -- one instance per template per project
);

-- Add persona assignment to tasks
ALTER TABLE tasks ADD COLUMN assigned_persona_id BLOB REFERENCES project_personas(id);

-- Create scoring rules table
CREATE TABLE scoring_rules (
    id              BLOB PRIMARY KEY,
    action_type     TEXT NOT NULL,
    task_size       TEXT NOT NULL DEFAULT 'standard',
    professionalism_points REAL NOT NULL DEFAULT 0.0,
    quality_points  REAL NOT NULL DEFAULT 0.0,
    description     TEXT NOT NULL DEFAULT '',
    CHECK (task_size IN ('small', 'standard')),
    CHECK (action_type IN ('task_completion', 'process_violation', 'quality_issue', 'kudos_base', 'wtf_base', 'delegation', 'tool_usage', 'git_workflow', 'testing', 'documentation'))
);

-- Create persona activities tracking table (high-level activities)
CREATE TABLE persona_activities (
    id                     BLOB PRIMARY KEY,
    project_persona_id     BLOB NOT NULL,
    task_id                BLOB,
    activity_type          TEXT NOT NULL,
    description            TEXT NOT NULL,
    professionalism_change REAL NOT NULL DEFAULT 0.0,
    quality_change         REAL NOT NULL DEFAULT 0.0,
    task_size              TEXT NOT NULL DEFAULT 'standard'
                              CHECK (task_size IN ('small', 'standard')),
    metadata               TEXT, -- JSON for additional data
    created_at             TEXT NOT NULL DEFAULT (datetime('now', 'subsec')),
    FOREIGN KEY (project_persona_id) REFERENCES project_personas(id) ON DELETE CASCADE,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE SET NULL,
    CHECK (activity_type IN ('task_assigned', 'task_completed', 'task_failed', 'kudos_received', 'wtf_received', 'process_violation', 'quality_issue', 'imported', 'score_adjustment', 'delegation', 'peer_review'))
);

-- Create detailed persona actions table (granular action tracking)
CREATE TABLE persona_actions (
    id                 BLOB PRIMARY KEY,
    project_persona_id BLOB NOT NULL,
    task_id            BLOB,
    activity_id        BLOB, -- link to parent activity
    action_type        TEXT NOT NULL,
    action_category    TEXT NOT NULL,
    tool_name          TEXT,
    parameters         TEXT, -- JSON of tool parameters
    result_status      TEXT NOT NULL DEFAULT 'success',
    execution_time_ms  INTEGER,
    description        TEXT NOT NULL,
    created_at         TEXT NOT NULL DEFAULT (datetime('now', 'subsec')),
    FOREIGN KEY (project_persona_id) REFERENCES project_personas(id) ON DELETE CASCADE,
    FOREIGN KEY (task_id) REFERENCES tasks(id) ON DELETE SET NULL,
    FOREIGN KEY (activity_id) REFERENCES persona_activities(id) ON DELETE SET NULL,
    CHECK (action_category IN ('file_operation', 'tool_usage', 'task_management', 'team_interaction', 'process_action', 'git_operation')),
    CHECK (action_type IN ('file_read', 'file_write', 'file_edit', 'file_delete', 'bash_command', 'git_commit', 'git_branch', 'git_pr', 'search_query', 'api_call', 'task_assigned', 'task_started', 'task_completed', 'task_delegated', 'kudos_given', 'wtf_issued', 'peer_review', 'collaboration', 'tests_run', 'build_executed')),
    CHECK (result_status IN ('success', 'failure', 'partial', 'cancelled'))
);

-- Create action artifacts table (specific outputs and changes)
CREATE TABLE action_artifacts (
    id           BLOB PRIMARY KEY,
    action_id    BLOB NOT NULL,
    artifact_type TEXT NOT NULL,
    file_path    TEXT,
    content_before TEXT,
    content_after TEXT,
    git_hash     TEXT,
    output_data  TEXT, -- JSON for command outputs, API responses, etc.
    size_bytes   INTEGER,
    created_at   TEXT NOT NULL DEFAULT (datetime('now', 'subsec')),
    FOREIGN KEY (action_id) REFERENCES persona_actions(id) ON DELETE CASCADE,
    CHECK (artifact_type IN ('file_change', 'command_output', 'git_diff', 'api_response', 'test_result', 'build_artifact'))
);

-- Create learning events table for pattern recognition
CREATE TABLE learning_events (
    id               BLOB PRIMARY KEY,
    project_persona_id BLOB,
    event_type       TEXT NOT NULL,
    category         TEXT NOT NULL,
    insight          TEXT NOT NULL,
    relevance_score  REAL NOT NULL DEFAULT 1.0,
    metadata         TEXT, -- JSON for additional context
    created_at       TEXT NOT NULL DEFAULT (datetime('now', 'subsec')),
    expires_at       TEXT, -- for aging mechanism
    FOREIGN KEY (project_persona_id) REFERENCES project_personas(id) ON DELETE SET NULL,
    CHECK (event_type IN ('excellence_callout', 'warning_callout', 'perfect_execution', 'critical_failure', 'pattern_recognition', 'team_learning')),
    CHECK (category IN ('process_compliance', 'quality_standards', 'collaboration', 'technical_skills', 'team_dynamics'))
);

-- Insert capabilities
INSERT INTO capabilities (id, name, category, description, keywords) VALUES
    (randomblob(16), 'task_delegation', 'management', 'Ability to analyze requirements and delegate to appropriate specialists', '["delegation", "assignment", "coordination", "project_management"]'),
    (randomblob(16), 'process_enforcement', 'management', 'Enforce team processes and quality standards', '["process", "compliance", "standards", "enforcement"]'),
    (randomblob(16), 'unlimited_kudos_wtf', 'management', 'Unlimited daily quota for Kudos/WTF recognition', '["kudos", "wtf", "recognition", "feedback"]'),
    (randomblob(16), 'team_coordination', 'management', 'Coordinate team activities and resolve conflicts', '["coordination", "team", "collaboration", "leadership"]'),
    (randomblob(16), 'requirements_analysis', 'analysis', 'Analyze and document business requirements', '["requirements", "analysis", "documentation", "stakeholders"]'),
    (randomblob(16), 'user_story_creation', 'analysis', 'Create detailed user stories and acceptance criteria', '["user_stories", "acceptance_criteria", "agile", "requirements"]'),
    (randomblob(16), 'specification_writing', 'analysis', 'Write technical and functional specifications', '["specifications", "documentation", "technical_writing"]'),
    (randomblob(16), 'architecture_design', 'architecture', 'Design system architecture and technical solutions', '["architecture", "design", "system_design", "scalability"]'),
    (randomblob(16), 'technical_decisions', 'architecture', 'Make informed technical decisions and trade-offs', '["technical_decisions", "trade_offs", "evaluation", "leadership"]'),
    (randomblob(16), 'scalability_planning', 'architecture', 'Plan for system scalability and performance', '["scalability", "performance", "planning", "optimization"]'),
    (randomblob(16), 'feature_implementation', 'implementation', 'Implement new features and functionality', '["implementation", "coding", "features", "development"]'),
    (randomblob(16), 'bug_fixing', 'implementation', 'Debug and fix software issues', '["debugging", "bug_fixing", "troubleshooting", "problem_solving"]'),
    (randomblob(16), 'test_writing', 'testing', 'Write comprehensive tests and test automation', '["testing", "test_automation", "unit_tests", "integration_tests"]'),
    (randomblob(16), 'code_review', 'implementation', 'Review code for quality and standards compliance', '["code_review", "quality", "standards", "peer_review"]'),
    (randomblob(16), 'infrastructure_management', 'devops', 'Manage system infrastructure and deployment', '["infrastructure", "deployment", "system_administration", "operations"]'),
    (randomblob(16), 'cicd_management', 'devops', 'Manage CI/CD pipelines and automation', '["ci_cd", "automation", "deployment", "pipelines"]'),
    (randomblob(16), 'schema_design', 'database', 'Design database schemas and data models', '["database", "schema", "data_modeling", "sql"]'),
    (randomblob(16), 'security_review', 'security', 'Review code and systems for security vulnerabilities', '["security", "vulnerability", "penetration_testing", "compliance"]'),
    (randomblob(16), 'ai_ml_integration', 'ai_ml', 'Integrate AI/ML capabilities and models', '["ai", "ml", "machine_learning", "models", "integration"]'),
    (randomblob(16), 'ui_design', 'design', 'Design user interfaces and user experiences', '["ui", "ux", "design", "user_interface", "user_experience"]'),
    (randomblob(16), 'test_strategy', 'qa', 'Develop comprehensive testing strategies', '["test_strategy", "qa", "quality_assurance", "testing_methodology"]');

-- Insert default Virtual Team persona templates
INSERT INTO persona_templates (id, name, role_type, default_instructions, description, capabilities, kudos_quota_daily, is_system) VALUES
    (randomblob(16), '@PM', 'pm', 'Project Manager responsible for task delegation, process compliance, and team coordination. Analyzes requirements and delegates to appropriate specialists. Never implements code directly - always delegates to technical roles. Enforces quality standards and process compliance.', 'Project Manager - coordinates team and delegates tasks', '["task_delegation", "process_enforcement", "unlimited_kudos_wtf", "team_coordination"]', -1, TRUE),
    (randomblob(16), '@Requirements-Engineer', 'requirements_engineer', 'Analyzes and documents requirements, creates user stories, and ensures clear specifications for development tasks. Works closely with stakeholders to understand business needs and translate them into technical requirements.', 'Requirements Engineer - defines and documents requirements', '["requirements_analysis", "user_story_creation", "specification_writing"]', 1, TRUE),
    (randomblob(16), '@Architect', 'architect', 'Designs system architecture, makes technical decisions, and ensures scalable solutions. Has 3 Kudos/WTF per day. Provides technical leadership and guides implementation decisions across the team.', 'System Architect - designs technical architecture', '["architecture_design", "technical_decisions", "scalability_planning"]', 3, TRUE),
    (randomblob(16), '@Developer', 'developer', 'Implements features, fixes bugs, writes tests, and follows coding standards. Focuses on clean, maintainable code and comprehensive testing. Participates in code reviews and technical discussions.', 'Software Developer - implements features and fixes', '["feature_implementation", "bug_fixing", "test_writing", "code_review"]', 1, TRUE),
    (randomblob(16), '@User-Role', 'user_role', 'Represents end-user perspective, provides feedback, and validates user experience requirements. Performs user acceptance testing and ensures solutions meet user needs.', 'User Representative - provides user perspective', '["user_feedback", "ux_validation", "requirement_verification"]', 1, TRUE),
    (randomblob(16), '@System-Engineer', 'system_engineer', 'Manages system infrastructure, deployment, and operational concerns. Ensures system reliability and performance. Handles system configuration and operational monitoring.', 'System Engineer - manages infrastructure and operations', '["infrastructure_management", "deployment", "system_reliability", "performance_optimization"]', 1, TRUE),
    (randomblob(16), '@DevOps-Engineer', 'devops_engineer', 'Handles CI/CD pipelines, deployment automation, and infrastructure as code. Bridges development and operations. Ensures smooth deployment processes and system automation.', 'DevOps Engineer - automates deployment and operations', '["cicd_management", "deployment_automation", "infrastructure_as_code", "monitoring"]', 1, TRUE),
    (randomblob(16), '@Database-Engineer', 'database_engineer', 'Designs database schemas, optimizes queries, and ensures data integrity and performance. Handles database migrations and data architecture decisions.', 'Database Engineer - manages data layer', '["schema_design", "query_optimization", "data_integrity", "performance_tuning"]', 1, TRUE),
    (randomblob(16), '@Security-Engineer', 'security_engineer', 'Reviews code for security vulnerabilities, implements security best practices, and ensures compliance. Conducts security assessments and implements security controls.', 'Security Engineer - ensures security and compliance', '["security_review", "vulnerability_assessment", "compliance_validation", "security_implementation"]', 1, TRUE),
    (randomblob(16), '@AI-Engineer', 'ai_engineer', 'Specializes in AI/ML integration, prompt engineering, and intelligent system features. Implements AI capabilities and optimizes model performance.', 'AI Engineer - develops AI/ML features', '["ai_ml_integration", "prompt_engineering", "model_optimization", "intelligent_features"]', 1, TRUE),
    (randomblob(16), '@Web-Designer', 'web_designer', 'Creates user interfaces, ensures design consistency, and focuses on user experience and accessibility. Designs responsive and accessible web interfaces.', 'Web Designer - creates UI and ensures UX', '["ui_design", "ux_optimization", "accessibility", "design_consistency"]', 1, TRUE),
    (randomblob(16), '@QA-Engineer', 'qa_engineer', 'Develops test strategies, creates test cases, and ensures quality standards across the application. Implements quality assurance processes and testing methodologies.', 'QA Engineer - ensures quality and testing', '["test_strategy", "test_case_creation", "quality_assurance", "testing_automation"]', 1, TRUE),
    (randomblob(16), '@Frontend-Tester', 'frontend_tester', 'Specializes in frontend testing, UI automation, and user interface validation. Ensures cross-browser compatibility and UI functionality.', 'Frontend Tester - validates UI and frontend', '["frontend_testing", "ui_automation", "user_interface_validation", "browser_testing"]', 1, TRUE),
    (randomblob(16), '@Backend-Tester', 'backend_tester', 'Focuses on API testing, integration testing, and backend system validation. Ensures backend reliability and performance through comprehensive testing.', 'Backend Tester - validates APIs and backend systems', '["api_testing", "integration_testing", "backend_validation", "performance_testing"]', 1, TRUE);

-- Insert scoring rules
INSERT INTO scoring_rules (id, action_type, task_size, professionalism_points, quality_points, description) VALUES
    (randomblob(16), 'task_completion', 'standard', 0.5, 0.5, 'Successfully completing a standard task'),
    (randomblob(16), 'task_completion', 'small', 0.25, 0.25, 'Successfully completing a small task'),
    (randomblob(16), 'process_violation', 'standard', -0.5, 0.0, 'Violating team processes on standard task'),
    (randomblob(16), 'process_violation', 'small', -0.25, 0.0, 'Violating team processes on small task'),
    (randomblob(16), 'quality_issue', 'standard', 0.0, -0.5, 'Quality issues in standard task'),
    (randomblob(16), 'quality_issue', 'small', 0.0, -0.25, 'Quality issues in small task'),
    (randomblob(16), 'kudos_base', 'standard', 1.0, 1.0, 'Base points for receiving Kudos on standard task'),
    (randomblob(16), 'kudos_base', 'small', 0.5, 0.5, 'Base points for receiving Kudos on small task'),
    (randomblob(16), 'wtf_base', 'standard', -1.0, -1.0, 'Base points for receiving WTF on standard task'),
    (randomblob(16), 'wtf_base', 'small', -0.5, -0.5, 'Base points for receiving WTF on small task'),
    (randomblob(16), 'delegation', 'standard', 0.25, 0.0, 'Proper task delegation'),
    (randomblob(16), 'tool_usage', 'standard', 0.1, 0.0, 'Effective tool usage'),
    (randomblob(16), 'git_workflow', 'standard', 0.25, 0.25, 'Following proper Git workflow'),
    (randomblob(16), 'testing', 'standard', 0.0, 0.25, 'Writing comprehensive tests'),
    (randomblob(16), 'documentation', 'standard', 0.25, 0.1, 'Creating quality documentation');

-- Create indexes for performance
CREATE INDEX idx_capabilities_category ON capabilities(category);
CREATE INDEX idx_persona_templates_role_type ON persona_templates(role_type);
CREATE INDEX idx_persona_templates_system ON persona_templates(is_system);
CREATE INDEX idx_project_personas_project_id ON project_personas(project_id);
CREATE INDEX idx_project_personas_template_id ON project_personas(template_id);
CREATE INDEX idx_project_personas_active ON project_personas(is_active);
CREATE INDEX idx_tasks_assigned_persona ON tasks(assigned_persona_id);
CREATE INDEX idx_persona_activities_project_persona_id ON persona_activities(project_persona_id);
CREATE INDEX idx_persona_activities_task_id ON persona_activities(task_id);
CREATE INDEX idx_persona_activities_created_at ON persona_activities(created_at);
CREATE INDEX idx_persona_actions_project_persona_id ON persona_actions(project_persona_id);
CREATE INDEX idx_persona_actions_task_id ON persona_actions(task_id);
CREATE INDEX idx_persona_actions_activity_id ON persona_actions(activity_id);
CREATE INDEX idx_persona_actions_action_type ON persona_actions(action_type);
CREATE INDEX idx_persona_actions_created_at ON persona_actions(created_at);
CREATE INDEX idx_action_artifacts_action_id ON action_artifacts(action_id);
CREATE INDEX idx_action_artifacts_artifact_type ON action_artifacts(artifact_type);
CREATE INDEX idx_learning_events_project_persona_id ON learning_events(project_persona_id);
CREATE INDEX idx_learning_events_event_type ON learning_events(event_type);
CREATE INDEX idx_learning_events_created_at ON learning_events(created_at);