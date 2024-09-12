use axum::{
    extract::{Path, rejection::JsonRejection},
    http::StatusCode,
    response::Html,
    routing::{get, post},
    Json, Router, Server,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::File,
    io::Read,
    net::SocketAddr,
    sync::{Arc, RwLock},
};
use tower::ServiceBuilder;
use tower_http::limit::RequestBodyLimitLayer;
use validator::{Validate, ValidationError};
use regex::Regex;

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
struct Payment {
    #[validate(length(min = 1))]
    transaction: String,
    #[validate(length(min = 1))]
    currency: String,
    provider: String,
    amount: i32,
    payment_dt: i64,
    bank: String,
    delivery_cost: i32,
    goods_total: i32,
    custom_fee: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
struct Delivery {
    #[validate(length(min = 1))]
    name: String,
    #[validate(custom = "validate_phone")]
    phone: String,
    #[validate(length(min = 1))]
    zip: String,
    #[validate(length(min = 1))]
    city: String,
    #[validate(length(min = 1))]
    address: String,
    #[validate(length(min = 1))]
    region: String,
    #[validate(email)]
    email: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
struct Item {
    chrt_id: i64,
    track_number: String,
    price: i32,
    rid: String,
    name: String,
    sale: i32,
    size: String,
    total_price: i32,
    nm_id: i64,
    brand: String,
    status: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate)]
struct Order {
    #[validate(length(min = 1))]
    order_uid: String,
    #[validate(length(min = 1))]
    track_number: String,
    entry: String,
    delivery: Delivery,
    payment: Payment,
    #[validate(length(min = 1))]
    items: Vec<Item>,
    locale: String,
    customer_id: String,
    delivery_service: String,
    shardkey: String,
    sm_id: i32,
    date_created: String,
    oof_shard: String,
}

type OrdersCache = Arc<RwLock<HashMap<String, Vec<Order>>>>;

fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    let re = Regex::new(r"^\+?[0-9]{7,15}$").unwrap();
    if re.is_match(phone) {
        Ok(())
    } else {
        Err(ValidationError::new("invalid phone number"))
    }
}

async fn home_page() -> Html<String> {
    Html(format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
        <meta charset="UTF-8">
        <meta name="viewport" content="width=device-width, initial-scale=1.0">
        <title>Order System</title>
        <link href="https://fonts.googleapis.com/css2?family=Roboto:wght@400;500;700&display=swap" rel="stylesheet">
        <style>
        body {{
            font-family: 'Roboto', sans-serif;
            background: #f4f4f9;
            padding: 20px;
            color: #333;
            display: flex;
            flex-direction: column;
            align-items: center;
        }}
        h1 {{
            font-size: 2.5rem;
            margin-bottom: 20px;
            color: #333;
        }}
        h2 {{
            margin-bottom: 10px;
            color: #555;
        }}
        .btn {{
            padding: 10px 20px;
            font-size: 16px;
            font-weight: 500;
            color: #fff;
            background: linear-gradient(135deg, #6a5acd, #836fff);
            border: none;
            border-radius: 5px;
            cursor: pointer;
            transition: background 0.3s ease-in-out;
        }}
        .btn:hover {{
            background: linear-gradient(135deg, #5a4acd, #735fff);
        }}
        #order_uid {{
            padding: 10px;
            font-size: 16px;
            border: 1px solid #ccc;
            border-radius: 4px;
            width: 300px;
            margin-bottom: 10px;
        }}
        #order_uid:focus {{
            border-color: #836fff;
            outline: none;
        }}
        #response {{
            margin-top: 20px;
            width: 100%;
            max-width: 600px;
            padding: 15px;
            background-color: #fff;
            border-radius: 6px;
            box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
            display: none; 
        }}
        .container {{
            display: flex;
            flex-direction: column;
            align-items: center;
            margin-top: 20px;
        }}
        </style>
        <script>
        async function fetchOrders() {{
            const response = await fetch('/orders');
            const data = await response.text();
            const responseDiv = document.getElementById('response');
            responseDiv.innerHTML = data;
            responseDiv.style.display = 'block';
            window.location.hash = "response";
        }}

        async function fetchOrderByUID() {{
            const orderUid = document.getElementById('order_uid').value;
            const response = await fetch('/order/' + orderUid);
            const data = await response.text();
            const responseDiv = document.getElementById('response');
            responseDiv.innerHTML = data;
            responseDiv.style.display = 'block';
            window.location.hash = "response";
        }}
        </script>
        </head>
        <body>
        
        <h1>Order Service</h1>
        
        <div class="container">
            <h2>Get All Orders</h2>
            <button class="btn" onclick="fetchOrders()">Get All Orders</button>
        
            <h2>Get Order by UID</h2>
            <input id="order_uid" type="text" placeholder="Enter Order UID" />
            <button class="btn" onclick="fetchOrderByUID()">Get Order</button>
        </div>
        
        <div id="response"></div>
        
        </body>
        </html>
        "#,
    ))
}

async fn get_order(order_uid: String, data: OrdersCache) -> Html<String> {
    let data = data.read().unwrap();
    if let Some(orders) = data.get(&order_uid) {
        let latest_order = orders.last().unwrap();
        let items_html: String = latest_order.items.iter().map(|item| {
            format!(
                r#"<li><strong>Item:</strong> {}<br/><strong>Price:</strong> {}<br/><strong>Brand:</strong> {}<br/><strong>Status:</strong> {}</li>"#,
                item.name, item.total_price, item.brand, item.status
            )
        }).collect();

        Html(format!(
                r#"<h1>Order Details</h1>
                <p><strong>Order UID:</strong> {}</p>
                <p><strong>Track Number:</strong> {}</p>
                <p><strong>Customer Name:</strong> {}</p>
                <p><strong>Customer Phone:</strong> {}</p>
                <p><strong>Address:</strong> {}, {}, {}</p>
                <h2>Payment Information</h2>
                <p><strong>Transaction:</strong> {}</p>
                <p><strong>Currency:</strong> {}</p>
                <p><strong>Amount:</strong> {}</p>
                <p><strong>Bank:</strong> {}</p>
                <h2>Items</h2>
                <ul>{}</ul>
                <h2>Date Created:</h2>
                <p>{}</p>"#,
            latest_order.order_uid, latest_order.track_number, latest_order.delivery.name, latest_order.delivery.phone, latest_order.delivery.address, latest_order.delivery.city, latest_order.delivery.region, latest_order.payment.transaction, latest_order.payment.currency, latest_order.payment.amount, latest_order.payment.bank, items_html, latest_order.date_created
        ))
    } else {
        Html(format!("<h1>No order found with UID: {}</h1>", order_uid))
    }
}

async fn list_orders(data: OrdersCache) -> Html<String> {
    let data = data.read().unwrap();
    if data.is_empty() {
        return Html("<h1>No orders available!</h1>".to_string());
    }

    let mut orders_html = String::new();
    for (order_uid, orders) in data.iter() {
        let latest_order = orders.last().unwrap();
        orders_html.push_str(&format!(
            r#"<h2>Order UID: {}</h2>
            <p><strong>Track Number:</strong> {}</p>
            <p><strong>Customer Name:</strong> {}</p>
            <p><strong>Total Amount:</strong> {}</p>
            <hr/>"#,
            order_uid, latest_order.track_number, latest_order.delivery.name, latest_order.payment.amount,
        ));
    }

    Html(format!("{}", orders_html))
}

async fn add_order(payload: Result<Json<Order>, JsonRejection>, data: OrdersCache) -> Result<StatusCode, (StatusCode, String)> {
    match payload {
        Ok(Json(order)) => {
            if let Err(validation_errors) = order.validate() {
                return Err((
                    StatusCode::BAD_REQUEST,
                    format!("Validation error: {:?}", validation_errors),
                ));
            }

            let mut data = data.write().unwrap();
            let orders = data.entry(order.order_uid.clone()).or_insert_with(Vec::new);
            orders.push(order);

            Ok(StatusCode::OK)
        }
        Err(rejection) => {
            Err((
                StatusCode::BAD_REQUEST,
                format!("Invalid JSON request: {:?}", rejection),
            ))
        }
    }
}

fn read_order_from_json_file(file_path: &str) -> Order {
    let mut file = File::open(file_path).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");

    let order: Order = serde_json::from_str(&contents).expect("Failed to parse JSON");
    order
}

#[tokio::main]
async fn main() {
    let orders: OrdersCache = Arc::new(RwLock::new(HashMap::new()));

    let order_from_file = read_order_from_json_file("model.json");
    {
        let mut data = orders.write().unwrap();
        data.entry(order_from_file.order_uid.clone()).or_insert_with(Vec::new).push(order_from_file);
    }

    let app = Router::new()
        .route("/", get(home_page))
        .route("/order", post({
            let orders = orders.clone();
            move |payload| async move { add_order(payload, orders.clone()).await }
        }))
        .route("/orders", get({
            let orders = orders.clone();
            move || async move { list_orders(orders.clone()).await }
        }))
        .route("/order/:order_uid", get({
            let orders = orders.clone();
            move |Path(order_uid): Path<String>| async move { get_order(order_uid, orders.clone()).await }
        }));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Server running on http://127.0.0.1:3000");

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}