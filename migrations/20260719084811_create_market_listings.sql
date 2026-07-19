CREATE TABLE market_listings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    seller_id VARCHAR(255) NOT NULL,
    item_id VARCHAR(255) NOT NULL,
    price INT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);