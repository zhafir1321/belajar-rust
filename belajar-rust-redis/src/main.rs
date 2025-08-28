fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use futures::StreamExt;
    use redis::{
        AsyncCommands, Client, Commands, RedisError, Value,
        aio::{MultiplexedConnection, PubSub},
        geo::{RadiusOptions, Unit},
        streams::{StreamReadOptions, StreamReadReply},
    };
    use std::{collections::HashMap, num::NonZero, time::Duration, vec};

    /*
    CLIENT
    - Hal pertama yang perlu kita lakukan saat ingin menggunakan Redis dari Rust,
    adalah membuat koneksi ke Rust
    - Untuk membuat koneksi ke Redis, kita perlu membuat object Client
     */
    #[test]
    fn test_connection() {
        let mut client = Client::open("redis://localhost:6379").unwrap();

        let _: () = client.set("name", "Zhafir").unwrap();

        let value: String = client.get("name").unwrap();
        println!("Value: {}", value);
    }

    /*
    ASYNC CLIENT
    - Library Redis juga menyediakan fitur Async jika kita ingin menggunakan Rust Async IO
    - Kita bisa pilih library Async yang akan kita gunakan, contohnya disini kita akan menggunakan Tokio
     */

    async fn get_client() -> Result<MultiplexedConnection, RedisError> {
        let client = Client::open("redis://localhost:6379")?;
        client.get_multiplexed_async_connection().await
    }

    #[tokio::test]
    async fn test_async_connection() -> Result<(), RedisError> {
        let mut con = get_client().await?;
        let _: () = con.set("name", "Zhafir").await?;
        let value: String = con.get("name").await?;
        println!("Value: {}", value);
        Ok(())
    }

    /*
    STRING
    - Struktur data yang sering digunakan di Redis adalah String
    - Nama - nama method nya hampir sama dengan perintah - perintah di Redis
     */

    #[tokio::test]
    async fn test_string() -> Result<(), RedisError> {
        let mut con = get_client().await?;
        let _: () = con.set_ex("name", "Zhafir", 2).await?;
        let value: String = con.get("name").await?;
        println!("Value: {}", value);

        tokio::time::sleep(Duration::from_secs(5)).await;

        let value: Result<String, RedisError> = con.get("name").await;
        println!("Value after expired: {:?}", value);
        Ok(())
    }

    /*
    LIST
     */

    #[tokio::test]
    async fn test_list() -> Result<(), RedisError> {
        let mut con = get_client().await?;

        let _: () = con.del("names").await?;
        let _: () = con.rpush("names", "Zhafir").await?;
        let _: () = con.rpush("names", "Rasyid").await?;
        let _: () = con.rpush("names", "Hafidz").await?;

        let len: i32 = con.llen("names").await?;
        println!("Length of names: {}", len);
        assert_eq!(3, len);

        let names: Vec<String> = con.lrange("names", 0, -1).await?;
        println!("Names: {:?}", names);
        assert_eq!(vec!["Zhafir", "Rasyid", "Hafidz"], names);

        let name: Vec<String> = con.lpop("names", NonZero::new(1)).await?;
        println!("Name popped: {:?}", name);
        assert_eq!(vec!["Zhafir"], name);

        let names: Vec<String> = con.lpop("names", NonZero::new(1)).await?;
        println!("Names after pop: {:?}", names);
        assert_eq!(vec!["Rasyid"], names);

        let names: Vec<String> = con.lpop("names", NonZero::new(1)).await?;
        println!("Names after pop: {:?}", names);
        assert_eq!(vec!["Hafidz"], names);

        Ok(())
    }

    /*
    SET
     */

    #[tokio::test]
    async fn test_set() -> Result<(), RedisError> {
        let mut con = get_client().await?;
        let _: () = con.del("names").await?;
        let _: () = con.sadd("names", "Zhafir").await?;
        let _: () = con.sadd("names", "Zhafir").await?;
        let _: () = con.sadd("names", "Zhafir").await?;
        let _: () = con.sadd("names", "Rasyid").await?;
        let _: () = con.sadd("names", "Rasyid").await?;
        let _: () = con.sadd("names", "Hafidz").await?;

        let len: i32 = con.scard("names").await?;
        println!("Length of names: {}", len);
        assert_eq!(3, len);

        let names: Vec<String> = con.smembers("names").await?;
        println!("Names: {:?}", names);
        assert_eq!(vec!["Zhafir", "Rasyid", "Hafidz"], names);

        Ok(())
    }

    /*
    SORTED SET
     */

    #[tokio::test]
    async fn test_sorted_set() -> Result<(), RedisError> {
        let mut con = get_client().await?;

        let _: () = con.del("names").await?;
        let _: () = con.zadd("names", "Zhafir", 100).await?;
        let _: () = con.zadd("names", "Rasyid", 80).await?;
        let _: () = con.zadd("names", "Hafidz", 95).await?;

        let len: i32 = con.zcard("names").await?;
        println!("Length of names: {}", len);
        assert_eq!(3, len);

        let names: Vec<String> = con.zrange("names", 0, -1).await?;
        println!("Names: {:?}", names);
        assert_eq!(vec!["Rasyid", "Hafidz", "Zhafir"], names);
        Ok(())
    }

    /*
    HASH
     */

    #[tokio::test]
    async fn test_hash() -> Result<(), RedisError> {
        let mut con = get_client().await?;

        let _: () = con.del("user:1").await?;
        let _: () = con.hset("user:1", "id", 1).await?;
        let _: () = con.hset("user:1", "name", "Zhafir").await?;
        let _: () = con.hset("user:1", "age", 30).await?;
        let _: () = con.hset("user:1", "email", "zhafir@example.com").await?;
        let _: () = con.hset("user:1", "city", "Jakarta").await?;

        let name: String = con.hget("user:1", "name").await?;
        let age: i32 = con.hget("user:1", "age").await?;
        let email: String = con.hget("user:1", "email").await?;
        let city: String = con.hget("user:1", "city").await?;
        println!(
            "Name: {}, Age: {}, Email: {}, City: {}",
            name, age, email, city
        );

        let user: HashMap<String, String> = con.hgetall("user:1").await?;
        println!("User: {:?}", user);
        assert_eq!("1", user.get("id").unwrap());
        assert_eq!("Zhafir", user.get("name").unwrap());
        assert_eq!("30", user.get("age").unwrap());
        assert_eq!("zhafir@example.com", user.get("email").unwrap());
        assert_eq!("Jakarta", user.get("city").unwrap());
        Ok(())
    }

    /*
    GEO POINT
     */

    #[tokio::test]
    async fn test_geo_point() -> Result<(), RedisError> {
        let mut con = get_client().await?;

        let _: () = con.del("sellers").await?;
        let _: () = con
            .geo_add("sellers", (106.822702, -6.177590, "Toko A"))
            .await?;
        let _: () = con
            .geo_add("sellers", (106.820889, -6.174964, "Toko B"))
            .await?;

        let distance: f64 = con
            .geo_dist("sellers", "Toko A", "Toko B", Unit::Kilometers)
            .await?;
        println!("Distance between Toko A and Toko B: {} km", distance);

        let result: Vec<String> = con
            .geo_radius(
                "sellers",
                106.821825,
                -6.175105,
                5.0,
                Unit::Kilometers,
                RadiusOptions::default(),
            )
            .await?;
        println!(
            "Sellers within 5 km of (106.821825, -6.175105): {:?}",
            result
        );

        assert_eq!(vec!["Toko A", "Toko B"], result);

        Ok(())
    }

    /*
    HYPER LOG LOG
     */

    #[tokio::test]
    async fn test_hyper_log_log() -> Result<(), RedisError> {
        let mut con = get_client().await?;

        let _: () = con.del("visitors").await?;
        let _: () = con
            .pfadd("visitors", ("Zhafir", "Rasyid", "Hafidz"))
            .await?;
        let _: () = con.pfadd("visitors", ("Zhafir", "Budi", "Wawan")).await?;
        let _: () = con
            .pfadd("visitors", ("Rasyid", "Yayat", "Muhammad"))
            .await?;

        let total: i32 = con.pfcount("visitors").await?;
        println!("Total unique visitors: {}", total);
        assert_eq!(7, total);
        Ok(())
    }

    /*
    PIPELINE
    - Kita bisa mengirim beberapa perintah secara langsung tanpa harus menunggu balasan satu per satu dari Redis
    - Hal ini juga bisa dilakukan menggunakan Rust menggunakan method `redis::pipe()`
    - Method `pipe()` akan mengembalikan object Pipeline yang bisa kita gunakan sebagai Redis Client yang akan dieksekusi semuanya menggunakan pipeline ketika memanggil method `exec_async()`
     */

    #[tokio::test]
    async fn test_pipeline() -> Result<(), RedisError> {
        let mut con = get_client().await?;

        redis::pipe()
            .set_ex("name", "Zhafir", 2)
            .set_ex("address", "Indonesia", 2)
            .exec_async(&mut con)
            .await?;

        let name: String = con.get("name").await?;
        let address: String = con.get("address").await?;
        println!("Name: {}, Address: {}", name, address);

        Ok(())
    }

    /*
    TRANSACTION
    - Redis bisa melakukan Transaction menggunakan perintah MULTI atau EXEC
    - Untuk menggunakan fitur Transaction, kita bisa menggunakan pipeline seperti sebelumnya,
    namun diawal pipeline kita perlu memanggil method `atomic()`
     */

    #[tokio::test]
    async fn test_transaction() -> Result<(), RedisError> {
        let mut con = get_client().await?;

        redis::pipe()
            .atomic()
            .set_ex("names", "Zhafir", 2)
            .set_ex("ages", 30, 2)
            .exec_async(&mut con)
            .await?;

        let name: String = con.get("names").await?;
        let age: i32 = con.get("ages").await?;
        println!("Name: {}, Age: {}", name, age);

        Ok(())
    }

    /*
    STREAM
     */

    #[tokio::test]
    async fn test_stream() -> Result<(), RedisError> {
        let mut con = get_client().await?;

        for i in 0..10 {
            let mut map: HashMap<&str, String> = HashMap::new();
            map.insert("name", format!("Zhafir {}", i));
            map.insert("address", "Indonesia".to_string());

            let _: () = con.xadd_map("members", "*", &map).await?;
        }

        Ok(())
    }

    #[tokio::test]
    async fn test_create_consumer() -> Result<(), RedisError> {
        let mut con = get_client().await?;

        let _: () = con.xgroup_create("members", "group-1", "0").await?;
        let _: () = con
            .xgroup_createconsumer("members", "group-1", "consumer-1")
            .await?;
        let _: () = con
            .xgroup_createconsumer("members", "group-1", "consumer-2")
            .await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_get_stream() -> Result<(), RedisError> {
        let mut con = get_client().await?;

        let setting = StreamReadOptions::default()
            .group("group-1", "consumer-1")
            .count(5)
            .block(3000);
        let result: StreamReadReply = con.xread_options(&["members"], &[">"], &setting).await?;

        for key in result.keys {
            for item in key.ids {
                let map: HashMap<String, Value> = item.map;
                let name: String = match map.get("name").unwrap() {
                    Value::BulkString(value) => String::from_utf8(value.to_vec())?,
                    _ => "".to_string(),
                };
                let address: String = match map.get("address").unwrap() {
                    Value::BulkString(value) => String::from_utf8(value.to_vec())?,
                    _ => "".to_string(),
                };

                println!("{:?}", name);
                println!("{:?}", address);
            }
        }
        Ok(())
    }

    /*
    PUBSUB
     */

    async fn get_pubsub() -> Result<PubSub, RedisError> {
        let client = Client::open("redis://localhost:6379")?;
        client.get_async_pubsub().await
    }

    #[tokio::test]
    async fn test_pubsub_subscribe() -> Result<(), RedisError> {
        let mut pubsub = get_pubsub().await?;

        let _: () = pubsub.subscribe("members").await?;
        let mut pubsub_stream = pubsub.on_message();
        let message: String = pubsub_stream.next().await.unwrap().get_payload()?;

        println!("Message received: {}", message);

        Ok(())
    }

    #[tokio::test]
    async fn test_pubsub_publish() -> Result<(), RedisError> {
        let mut con = get_client().await?;
        let _: () = con
            .publish("members", "Zhafir Rasyid Muhammad Hafidz")
            .await?;
        Ok(())
    }
}
