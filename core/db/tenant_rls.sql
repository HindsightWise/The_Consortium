-- SOP 4.1: PostgreSQL Row-Level Security (RLS) & Multi-Tenancy Sandbox
-- SOP 3.5.3: CPRA 2026 ADMT Opt-Out Boolean

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Base Table with Tenant Enforcement
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    admt_opt_out BOOLEAN DEFAULT FALSE, -- CPRA Mandate: Automated Decision Making Opt-Out
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE core_data (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES users(tenant_id),
    payload JSONB NOT NULL,
    structured_field VARCHAR(255) NOT NULL, -- SOP 4.1: Banned flat JSON blobs. 
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Force Row-Level Security on all tables
ALTER TABLE users ENABLE ROW LEVEL SECURITY;
ALTER TABLE core_data ENABLE ROW LEVEL SECURITY;

-- Policy ensuring a tenant can only query/modify their own rows.
CREATE POLICY tenant_isolation_users ON users
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant')::UUID);

CREATE POLICY tenant_isolation_data ON core_data
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant')::UUID);

-- SOP 3.5.1: Stripe Tax/Avalara placeholder schema for Transactions
CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    tenant_id UUID NOT NULL REFERENCES users(tenant_id),
    amount_cents INTEGER NOT NULL,
    sales_tax_cents INTEGER NOT NULL, -- Wayfair 2018 compliance strict numeric storage
    destination_state VARCHAR(2) NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

ALTER TABLE transactions ENABLE ROW LEVEL SECURITY;
CREATE POLICY tenant_isolation_transactions ON transactions
    FOR ALL
    USING (tenant_id = current_setting('app.current_tenant')::UUID);
