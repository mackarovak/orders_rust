use reqwest::Client;
use serde::{Deserialize, Serialize};

// Структуры для создания нового заказа
#[derive(Serialize, Deserialize)]
struct Payment {
    transaction: String,
    currency: String,
    provider: String,
    amount: i32,
    payment_dt: i64,
    bank: String,
    delivery_cost: i32,
    goods_total: i32,
    custom_fee: i32,
}

#[derive(Serialize, Deserialize)]
struct Delivery {
    name: String,
    phone: String,
    zip: String,
    city: String,
    address: String,
    region: String,
    email: String,
}

#[derive(Serialize, Deserialize)]
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

#[derive(Serialize, Deserialize)]
struct NewOrder {
    order_uid: String,
    track_number: String,
    entry: String,
    delivery: Delivery,
    payment: Payment,
    items: Vec<Item>,
    locale: String,
    customer_id: String,
    delivery_service: String,
    shardkey: String,
    sm_id: i32,
    date_created: String,
    oof_shard: String,
}

#[tokio::main]
async fn main() {
    // HTTP-клиент
    let client = Client::new();
    
    // Создаём новый заказ
    let new_order = NewOrder {
        order_uid: "67890".to_string(),
        track_number: "TRACK002".to_string(),
        entry: "".to_string(),
        delivery: Delivery {
            name: "Jane Smith".to_string(),
            phone: "987654321".to_string(),
            zip: "654321".to_string(),
            city: "Another City".to_string(),
            address: "Another Street, 2".to_string(),
            region: "Another Region".to_string(),
            email: "jane@example.com".to_string(),
        },
        payment: Payment {
            transaction: "TRANSACTION002".to_string(),
            currency: "USD".to_string(),
            provider: "Another Bank".to_string(),
            amount: 20000,
            payment_dt: 1633024900,
            bank: "Another Bank".to_string(),
            delivery_cost: 1000,
            goods_total: 19000,
            custom_fee: 100,
        },
        items: vec![Item {
            chrt_id: 2,
            track_number: "TRACK_ITEM002".to_string(),
            price: 10000,
            rid: "RID002".to_string(),
            name: "Item 2".to_string(),
            sale: 15,
            size: "L".to_string(),
            total_price: 8500,
            nm_id: 654321,
            brand: "Another Brand".to_string(),
            status: 1,
        }],
        locale: "en".to_string(),
        customer_id: "CUSTOMER002".to_string(),
        delivery_service: "FedEx".to_string(),
        shardkey: "".to_string(),
        sm_id: 2,
        date_created: "2021-10-03T11:00:00Z".to_string(),
        oof_shard: "".to_string(),
    };

    // Отправляем POST-запрос на сервер
    let res = client
        .post("http://127.0.0.1:3000/order")  // Используем порт 3000
        .json(&new_order)
        .send()
        .await
        .expect("Failed to send request");

    // Выводим ответ от сервера
    println!("Response: {:?}", res.text().await.unwrap());
}