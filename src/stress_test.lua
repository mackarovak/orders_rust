-- stress_test.lua
local orders = {}

wrk.method = "POST"
wrk.body   = [[
{
  "order_uid": "load-test-order-001",
  "track_number": "LOADTN123456",
  "entry": "entry-test",
  "delivery": {
    "name": "John Doe",
    "phone": "+1234567890",
    "zip": "12345",
    "city": "New York",
    "address": "123 Main St",
    "region": "NY",
    "email": "john@example.com"
  },
  "payment": {
    "transaction": "LOADT123456",
    "currency": "USD",
    "provider": "ProviderTest",
    "amount": 100,
    "payment_dt": 1234567890,
    "bank": "Test Bank",
    "delivery_cost": 10,
    "goods_total": 90,
    "custom_fee": 5
  },
  "items": [
    {
      "chrt_id": 1,
      "track_number": "LOADTRACK1",
      "price": 10,
      "rid": "LOADRID1",
      "name": "Item1",
      "sale": 1,
      "size": "M",
      "total_price": 9,
      "nm_id": 1001,
      "brand": "Brand1",
      "status": 0
    }
  ],
  "locale": "en-US",
  "customer_id": "cust123",
  "delivery_service": "ServiceTest",
  "shardkey": "shardkey-test",
  "sm_id": 1,
  "date_created": "2023-01-01T00:00:00Z",
  "oof_shard": "oof-shard-test"
}
]]
wrk.headers["Content-Type"] = "application/json"

local counter = 0

request = function()
  local path = "/order"
  if counter % 2 == 0 then
    wrk.method = "POST"
    wrk.body   = orders[counter]
  else
    wrk.method = "GET"
    path = "/orders"
  end
  counter = counter + 1
  return wrk.format(nil, path)
end