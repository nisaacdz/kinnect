CREATE TABLE Users (
    id SERIAL PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    profile_photo_url TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE UserAuthProviders (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES Users(id) ON DELETE CASCADE,
    provider VARCHAR(50) NOT NULL,
    provider_user_id VARCHAR(255) NOT NULL,
    access_token TEXT,
    refresh_token TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (provider, provider_user_id)
);

CREATE TABLE TreeVisibilityLevels (
    id SERIAL PRIMARY KEY,
    level_name VARCHAR(50) UNIQUE NOT NULL
);

CREATE TABLE FamilyTrees (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT,
    creator_id INT NOT NULL REFERENCES Users(id) ON DELETE CASCADE,
    root_node_id INT NOT NULL,
    visibility_level_id INT NOT NULL REFERENCES TreeVisibilityLevels(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_root_node FOREIGN KEY (root_node_id) REFERENCES Nodes(id) ON DELETE CASCADE
);

CREATE TABLE ForkedFamilyTrees (
    id SERIAL PRIMARY KEY,
    original_tree_id INT NOT NULL REFERENCES FamilyTrees(id) ON DELETE CASCADE,
    forked_by INT NOT NULL REFERENCES Users(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE Nodes (
    id SERIAL PRIMARY KEY,
    family_tree_id INT NOT NULL REFERENCES FamilyTrees(id) ON DELETE CASCADE,
    name VARCHAR(100) NOT NULL,
    photo_url TEXT,
    description TEXT,
    alias VARCHAR(100),
    user_id INT REFERENCES Users(id) ON DELETE
    SET
        NULL,
        parent_node_id INT,
        parent_relation VARCHAR(50),
        created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
        CONSTRAINT fk_parent_node FOREIGN KEY (parent_node_id, family_tree_id) REFERENCES Nodes(id, family_tree_id) ON DELETE
    SET
        NULL
);

CREATE INDEX idx_nodes_family_tree_id ON Nodes (family_tree_id);

CREATE TABLE Admins (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES Users(id) ON DELETE CASCADE,
    family_tree_id INT NOT NULL REFERENCES FamilyTrees(id) ON DELETE CASCADE,
    permissioned_subtree_id INT NOT NULL REFERENCES Nodes(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_admins_family_tree_id ON Admins (family_tree_id);

CREATE TABLE MergeRequestStatuses (
    id SERIAL PRIMARY KEY,
    status_name VARCHAR(50) UNIQUE NOT NULL
);

CREATE TABLE MergeRequests (
    id SERIAL PRIMARY KEY,
    fork_id INT NOT NULL REFERENCES FamilyTrees(id) ON DELETE CASCADE,
    status_id INT NOT NULL REFERENCES MergeRequestStatuses(id),
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE LoadNodeRequests (
    id SERIAL PRIMARY KEY,
    user_id INT NOT NULL REFERENCES Users(id) ON DELETE CASCADE,
    node_id INT NOT NULL REFERENCES Nodes(id) ON DELETE CASCADE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (user_id, node_id)
);