-- Simple migration without extensions (migrations/001_create_tasks_simple.sql)
-- This works with any PostgreSQL installation

CREATE TABLE tasks (
                       id UUID PRIMARY KEY,  -- UUIDs generated in Rust code
                       name VARCHAR(255) NOT NULL,
                       description TEXT,
                       status VARCHAR(50) NOT NULL DEFAULT 'pending'
                           CHECK (status IN ('pending', 'in_progress', 'completed', 'cancelled')),
                       created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                       updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                       due_date TIMESTAMPTZ,
                       frequency VARCHAR(50) NOT NULL DEFAULT 'none'
                           CHECK (frequency IN ('none', 'daily', 'weekly', 'monthly', 'yearly', 'custom')),
                       recurrence_date TIMESTAMPTZ
);

-- Basic indexes (no GIN indexes)
CREATE INDEX idx_tasks_status ON tasks(status);
CREATE INDEX idx_tasks_due_date ON tasks(due_date);
CREATE INDEX idx_tasks_created_at ON tasks(created_at);
CREATE INDEX idx_tasks_frequency ON tasks(frequency);
CREATE INDEX idx_tasks_due_date_status ON tasks(due_date, status);
CREATE INDEX idx_tasks_name ON tasks(name);
CREATE INDEX idx_tasks_description ON tasks(description);

-- Add a trigger to update updated_at automatically
CREATE OR REPLACE FUNCTION update_updated_at_column()
    RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TRIGGER update_tasks_updated_at
    BEFORE UPDATE ON tasks
    FOR EACH ROW
EXECUTE FUNCTION update_updated_at_column();

-- Insert sample data with hardcoded UUIDs
INSERT INTO tasks (id, name, description, status, due_date, frequency) VALUES
                                                                           ('550e8400-e29b-41d4-a716-446655440001', 'Welcome to Todo Tracker! 🎉', 'This is your first task. You can edit, complete, or delete it.', 'pending', NOW() + INTERVAL '1 hour', 'none'),
                                                                           ('550e8400-e29b-41d4-a716-446655440002', 'Daily standup meeting 👥', 'Team synchronization meeting every morning', 'pending', NOW() + INTERVAL '1 day', 'daily'),
                                                                           ('550e8400-e29b-41d4-a716-446655440003', 'Weekly planning session 📋', 'Plan upcoming work and review progress', 'pending', NOW() + INTERVAL '2 days', 'weekly'),
                                                                           ('550e8400-e29b-41d4-a716-446655440004', 'Monthly report 📊', 'Compile and submit monthly progress report', 'pending', NOW() + INTERVAL '1 week', 'monthly'),
                                                                           ('550e8400-e29b-41d4-a716-446655440005', 'Take a coffee break ☕', 'Remember to take regular breaks!', 'completed', NOW() - INTERVAL '2 hours', 'none'),
                                                                           ('550e8400-e29b-41d4-a716-446655440006', 'Review code changes', 'Check pull requests and provide feedback', 'in_progress', NOW() + INTERVAL '3 hours', 'none'),
                                                                           ('550e8400-e29b-41d4-a716-446655440007', 'Update documentation 📝', 'Keep project documentation current', 'pending', NOW() + INTERVAL '5 days', 'none');