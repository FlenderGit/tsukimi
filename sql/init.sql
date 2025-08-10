-- Extension pour générer des UUIDs
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Table des utilisateurs
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Table des moteurs/engines
CREATE TABLE engines (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    name VARCHAR(100) NOT NULL,
    description TEXT,
    current_version VARCHAR(20),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Index pour les recherches fréquentes
    CONSTRAINT engines_name_unique UNIQUE(name)
);

-- Table des versions d'engines
CREATE TABLE engine_versions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    engine_id UUID NOT NULL REFERENCES engines(id) ON DELETE CASCADE,
    version VARCHAR(20) NOT NULL,
    description TEXT,
    is_active BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    -- Contraintes
    CONSTRAINT engine_versions_unique UNIQUE(engine_id, version)
);

-- Index pour optimiser les requêtes
CREATE INDEX idx_engine_versions_engine_id ON engine_versions(engine_id);
CREATE INDEX idx_engine_versions_active ON engine_versions(is_active) WHERE is_active = true;
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_username ON users(username);

-- Fonction pour mettre à jour automatiquement updated_at
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Triggers pour updated_at
CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER update_engines_updated_at
    BEFORE UPDATE ON engines
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

-- Fonction pour maintenir current_version dans engines
CREATE OR REPLACE FUNCTION update_engine_current_version()
RETURNS TRIGGER AS $$
BEGIN
    IF NEW.is_active = true THEN
        -- Désactiver les autres versions actives pour ce moteur
        UPDATE engine_versions
        SET is_active = false
        WHERE engine_id = NEW.engine_id AND id != NEW.id AND is_active = true;

        -- Mettre à jour la version courante dans engines
        UPDATE engines
        SET current_version = NEW.version, updated_at = NOW()
        WHERE id = NEW.engine_id;
    END IF;

    RETURN NEW;
END;
$$ language 'plpgsql';

-- Trigger pour maintenir la cohérence
CREATE TRIGGER update_engine_current_version_trigger
    AFTER INSERT OR UPDATE ON engine_versions
    FOR EACH ROW
    WHEN (NEW.is_active = true)
    EXECUTE FUNCTION update_engine_current_version();

-- Contrainte pour s'assurer qu'il n'y a qu'une version active par engine
CREATE UNIQUE INDEX idx_engine_versions_single_active
ON engine_versions(engine_id)
WHERE is_active = true;

-- Données d'exemple
INSERT INTO users (username, email) VALUES
('admin', 'admin@example.com'),
('developer', 'dev@example.com');

INSERT INTO engines (name, description) VALUES
('PostgreSQL', 'Système de gestion de base de données relationnelle'),
('Redis', 'Base de données en mémoire pour le cache et les messages');

INSERT INTO engine_versions (engine_id, version, description, is_active)
SELECT
    e.id,
    '15.4',
    'Version stable de PostgreSQL avec améliorations de performance',
    true
FROM engines e WHERE e.name = 'PostgreSQL';

INSERT INTO engine_versions (engine_id, version, description, is_active)
SELECT
    e.id,
    '7.2',
    'Version récente de Redis avec nouvelles fonctionnalités',
    true
FROM engines e WHERE e.name = 'Redis';
