fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::cell::{Ref, RefCell};
    use std::sync::atomic::{AtomicI32, Ordering};
    use std::sync::{Arc, Barrier, Mutex, Once};
    /*
    THREAD
    Saat kita menjalankan aplikasi, aplikasi akan dijalankan dalam process, process akan diatur oleh sistem operasi.
    Dalam process, kita bisa membuat thread untuk menjalankan kode secara parallel dan asynchronous.
    Di Rust, kita bisa menggunakan module `std::thread` untuk membuat thread.

    MEMBUAT THREAD
    Untuk membuat thread baru yang berjalan secara parallel dan async, kita bisa menggunakan fungsi `std::thread::spawn(<closure>)`.
     */
    use std::time::Duration;
    use std::{thread, vec};

    use tokio::runtime::Runtime; // Import the thread module // Import the Duration type for sleep

    /*
    MENJALANKAN UNIT TEST
    Secara default, semua output dari unit test akan ditangkap oleh Rust Test Runner.
    Saat kita gunakan parameter `--show-output`, maka output akan ditampilkan setelah test selesai.
    Kadang, kita ingin menampilkan output secara real-time di terminal, kita bisa disable fitur capture untuk menangkap output dengan parameter `--nocapture`.
    Contoh:
        cargo test <nama_module>::<nama_test_function> -- --nocapture
     */
    #[test]
    fn test_create_thread() {
        thread::spawn(|| {
            // Spawn a new thread
            for i in 0..=5 {
                // Loop from 0 to 5
                println!("Counter: {}", i);
                thread::sleep(Duration::from_secs(1)); // Sleep for 1 second
            }
        });

        println!("Application done");
        thread::sleep(Duration::from_secs(7)); // Wait for the spawned thread to finish
    }

    /*
    JOIN THREAD
    Saat kita menjalankan thread menggunakan spawn function, maka dia akan mengembalikan data `JoinHandle<T>`.
    `JoinHandle<T>` bisa digunakan untuk melakukan join thread dengan memanggil method `join()`.
    Method `join()` akan mengembalikan `Result<T>`, sesuai dengan return dari threadnya.
     */
    #[test]
    fn test_join_thread() {
        let handle = thread::spawn(|| {
            let mut counter = 0;
            for i in 0..=5 {
                println!("Counter: {}", i);
                thread::sleep(Duration::from_secs(1)); // Sleep for 1 second
                counter += 1;
            }
            return counter; // Return the final counter value
        });

        println!("Waiting handle to finish");

        let result = handle.join();
        match result {
            Ok(value) => println!("Thread finished with value: {}", value),
            Err(e) => println!("Thread panicked: {:?}", e),
        }

        println!("Application done");
        thread::sleep(Duration::from_secs(7)); // Wait for the spawned thread to finish
    }

    fn calculate() -> i32 {
        let mut counter = 0;
        let current = thread::current();
        for i in 0..=5 {
            match current.name() {
                None => {
                    println!("{:?} : Counter : {}", current.id(), i)
                }
                Some(name) => {
                    println!("{} : Counter : {}", name, i)
                }
            }
        }
        return counter;
    }

    /*
    KEUNTUNGAN MENGGUNAKAN THREAD
    - Misal kita butuh melakukan dua kalkulasi berat, jika kita lakukan tanpa menggunakan thread, artinya kode akan dieksekusi secara synchronous dan sequential.
    - Jika tiap kalkulasi membutuhkan waktu misal 5 detik, maka kita butuh 10 detik untuk menyelesaikan tiap kalkulasi.
    - Namun jika kita jalankan menggunakan thread, artinya kalkulasi akan dijalankan secara asynchronous dan parallel, sehingga bisa jadi total waktu untuk menyelesaikan seluruh kalkulasi, hanya butuh waktu 5 detik.
     */
    #[test]
    fn test_sequential() {
        let result1 = calculate(); // First calculation
        let result2 = calculate(); // Second calculation
        println!("Result 1: {}", result1);
        println!("Result 2: {}", result2);
        println!("Application done");
    }

    #[test]
    fn test_parallel() {
        let handle1 = thread::spawn(|| calculate()); // First calculation in a thread
        let handle2 = thread::spawn(|| calculate()); // Second calculation in a thread

        let result1 = handle1.join();
        let result2 = handle2.join();

        match result1 {
            Ok(counter) => {
                println!("Thread 1 finished with value: {}", counter);
            }
            Err(error) => {
                println!("Thread 1 panicked: {:?}", error);
            }
        }

        match result2 {
            Ok(counter) => {
                println!("Thread 2 finished with value: {}", counter);
            }
            Err(error) => {
                println!("Thread 2 panicked: {:?}", error);
            }
        }
        println!("Application done");
    }

    /*
    CLOSURE
    - Saat kita menjalankan thread, kita menggunakan `spawn()` function, dimana parameternya adalah function.
    - Biasanya, kita akan menggunakan function dalam bentuk closure.
    - Saat kita menggunakan variable dari luar closure, hal ini diperbolehkan.
    - Namun, jika closure tersebut dikirim sebagai parameter di function lain, contoh di `spawn()`, maka Rust melarang itu,
    karena variable yang digunakan oleh closure tersebut harus dipindahkan ownership-nya ke closure.
     */

    /*
    THREAD FACTORY
    - Saat kita membuat thread, sebenernya kita menggunakan Thread Factory, yaitu object untuk membuat thread.
    - Secara default, Rust sudah membuatkan default Thread Factory, dan ketika menggunakan `thread::spawn()`, maka kita akan menggunakan deafault Thread Factory yang disediakan oleh Rust.
    - Namun, kita juga bisa membuat Thread Factory secara manual.
    - Hal ini mungkin dibutuhkan ketika kita ingin mengatur konfigurasi Thread Factory, atau nanti menggunakan library/ framework yang membutuhkan Thread Factory.
     */

    #[test]
    fn test_thread_factory() {
        let factory = thread::Builder::new().name("My Thread".to_string());

        let handler = factory.spawn(calculate).expect("Failed to create thread");

        let result = handler.join().unwrap();
        println!("Thread finished with result: {}", result);
    }

    /*
    THREAD COMMUNICATION
    Menggunakan konsep Channel untuk berkomunikasi antar thread seperti Go

    CHANNEL
    - Struktur data mirip seperti antrian (queue), dimana thread bisa mengirim data ke channel dan bisa menerima data dari channel
    - Jadi antar thread tidak ada komunikasi secara langsung melainkan melalui channel
    - Di dalam channel, terdapat dua pihak yaitu Sender dan Receiver
    - Thread dalam satu waktu bisa saja berperan sebagai Sender sekaligus Receiver
    - Channel di Rust direpresentasikan dalam module mpsc (Multi Producer, Single Consumer)
     */

    #[test]
    fn test_channel() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle1 = thread::spawn(move || {
            thread::sleep(Duration::from_secs(2));
            sender.send("Hello from thread".to_string())
        });

        let handle2 = thread::spawn(move || {
            let message = receiver.recv().unwrap();
            println!("Received message: {}", message);
        });

        let _ = handle1.join();
        let _ = handle2.join();
    }

    #[test]
    fn test_channel_queue() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();
        let handle1 = thread::spawn(move || {
            for i in 0..5 {
                thread::sleep(Duration::from_secs(2));
                sender.send("Hello from thread".to_string());
            }
            sender.send("Done".to_string());
        });

        let handle2 = thread::spawn(move || {
            loop {
                let message = receiver.recv().unwrap();
                if message == "Done" {
                    break;
                }
                println!("Received message: {}", message);
            }
        });

        let _ = handle1.join();
        let _ = handle2.join();
    }

    /*
    CHANNEL LIFE CYCLE
    - Saat membuat channel, secara otomatis akan dibuatkan Sender dan Receiver
    - Saat life cycle Sender berakhir, dan Sender dihapus dari memori, secara otomatis kita tidak akan bisa menerima data apapun dari Receiver
    - Oleh karena itu, sebenernya kita tidak perlu membuat kode break seperti pada test function sebelumnya
    - Receiver merupakan implementasi dari Iterator, sehingga kita bisa lakukan iterasi menggunakan for loop
    - Begitu juga sebaliknya, ketika life cycle Receiver sudah berakhir, saat kita mengirim ke Sender, maka akan terjadi error
     */

    #[test]
    fn test_channel_iterator() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        let handle1 = thread::spawn(move || {
            for i in 0..5 {
                thread::sleep(Duration::from_secs(1));
                sender.send(format!("Message {}", i)).unwrap();
            }
        });

        let handle2 = thread::spawn(move || {
            for message in receiver.iter() {
                println!("Received message: {}", message);
            }
        });

        let _ = handle1.join();
        let _ = handle2.join();
    }

    /*
    MULTI SENDER
    - Caranya adalah kita bisa melakukan Clone data Sender, secara otomatis Sender hasil clone akan mengirim ke Receiver yang sama
     */

    #[test]
    fn test_channel_multi_sender() {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();
        let sender2 = sender.clone();

        let handle1 = thread::spawn(move || {
            for i in 0..5 {
                thread::sleep(Duration::from_secs(1));
                sender
                    .send(format!("Message from sender 1: {}", i))
                    .unwrap();
            }
        });

        let handle3 = thread::spawn(move || {
            for i in 0..5 {
                thread::sleep(Duration::from_secs(1));
                sender2
                    .send(format!("Message from sender 2: {}", i))
                    .unwrap();
            }
        });

        let handle2 = thread::spawn(move || {
            for value in receiver.iter() {
                println!("Received: {}", value);
            }
        });

        let _ = handle1.join();
        let _ = handle2.join();
        let _ = handle3.join();
    }

    /*
    RACE CONDITION
    - Salah satu masalah ketika kita membuat aplikasi berbasis multi thread adalah, masalah Race Condition
    - Race Condition adalah kejadian dimana dua atau lebih thread mengubah ke mutable data yang sama
    - Ketika cara mengubahnya salah, maka bisa terjadi yang masalah Race Condition, sehingga hasil data tidak sesuai dengan yang kita inginkan
     */

    static mut COUNTER: i32 = 0;
    #[test]
    fn test_race_condition() {
        let mut handles = vec![];
        for _ in 0..10 {
            let handle = thread::spawn(|| unsafe {
                for _ in 0..1000000 {
                    COUNTER += 1;
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        println!("Counter: {}", unsafe { COUNTER });
    }

    /*
    BAGAIMANA CARA MENGATASI RACE CONDITION
    - Menggunakan Atomic
    - Menggunakan Lock
     */

    /*
    ATOMIC
    - Atomic merupakan tipe data yang digunakan untuk sharing untuk beberapa thread
    - Atomic sendiri merupakan tipe data yang membungkus tipe data aslinya
    - Kita bisa pilih jenis tipe data Atomic, sesuai dengan tipe data aslinya yang akan kita gunakan
    - Tipe data Atomic digaransi aman terhadap Race Condition
     */

    #[test]
    fn test_atomic() {
        use std::sync::atomic::{AtomicI32, Ordering};

        static counter: AtomicI32 = AtomicI32::new(0);

        let mut handles = vec![];
        for _ in 0..10 {
            let handle = thread::spawn(|| {
                for _ in 0..1000000 {
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        println!("Counter: {}", counter.load(Ordering::Relaxed));
    }

    /*
    ATOMIC REFERENCE
    - Salah satu problem ketika sharing data menggunakan multi thread di Rust adalah,
    ownership dari data harus dipindahkan ke thread, sedangkan dalam satu waktu,
    hanya boleh satu thread yang own data tersebut.
    - Oleh karena itu, padak kode Atomic sebelumnya, kita gunakan static agar scope nya global,
    namun kadang tidak semua kasus kita bisa menggunakan static
    - Rust menyediakan ARC (Atomic Reference Counted), yaitu tipe data yang bisa digunakan untuk membuat reference ke data lain,
    tipe ini mirip seperti tipe Rc, namun karena semua operasi ARC itu Atomic, oleh karena itu operasinya lebih mahal tapi keuntungannya adalah thread safe
     */

    #[test]
    fn test_atomic_reference() {
        let counter: Arc<AtomicI32> = Arc::new(AtomicI32::new(0));

        let mut handles = vec![];
        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..1000000 {
                    counter_clone.fetch_add(1, Ordering::Relaxed);
                }
            });
            handles.push(handle);
        }
        for handle in handles {
            handle.join().unwrap();
        }
        println!("Counter: {}", counter.load(Ordering::Relaxed));
    }

    /*
    MUTEX
    - Mutex (Mutual Exclusion) yaitu tipe data yang digunakan untuk melindungi data yang di-sharing ke lebih dari satu thread
    - Mutex akan memblok thread dan menunggu sampai lock (kunci) tersedia
    - Kita bisa menggunakan method lock() pada Mutex untuk menunggu sampai mendapatkan data, dan setelah data keluar dari scope,
    maka lock (kunci) akan dikembalikan ke Mutex sehingga thread lain bisa mengambil lock (kunci)nya.
     */

    #[test]
    fn test_mutex() {
        let counter: Arc<Mutex<i32>> = Arc::new(Mutex::new(0));

        let mut handles = vec![];
        for _ in 0..10 {
            let counter_clone = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..1000000 {
                    let mut data = counter_clone.lock().unwrap();
                    *data += 1;
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
        println!("Counter: {}", counter.lock().unwrap());
    }

    /*
    THREAD LOCAL
    - Rust memiliki fitur untuk menyimpan data di Thread bernama Thread Local
    - Konsel Thread Local di Rust mirip seperti di Java, dimana alur hidup data akan mengikuti Thread,
    jika Thread selesai, maka data di Thread Local akan di drop
    - Hal ini cocok ketika kita ingin membuat data yang memang ingin digunakan dalam scope thread selama thread tersebut aktif,
    dan tidak bertukar dengan thread lain

    MEMBUAT DATA DI THREAD LOCAL
    - Untuk membuat data di Thread Local, kita harus buat menggunakan macro thread_local!
    - Kita bisa tentukan menggunakan Cell atau RefCell, tergantung apakah tipe datanya mutable atau tidak
     */

    thread_local! {
       pub static NAME: RefCell<String> = RefCell::new("Default".to_string());
    }

    #[test]
    fn test_thread_local() {
        let handle = thread::spawn(|| {
            NAME.with_borrow_mut(|name| {
                *name = "Zhafir".to_string();
            });

            NAME.with_borrow(|name| {
                println!("Name: {}", name);
            });
        });
        handle.join();
        NAME.with_borrow(|name| {
            println!("Name: {}", name);
        });
    }

    /*
    THREAD PANIC
    - Ketika thread mengalami Thread Panic, maka thread tersebut akan berhenti, tapi tidak akan menghentikan thread lainnya
    - Ketika menjalankan thread baru, dan terjadi panic pada thread tersebut, maka thread utama (main) tidak akan berhenti, karena berbeda thread
    - Kecuali jika terjadi panic di thread utama (main), otomatis thread utama akan berhenti
     */

    #[test]
    fn test_thread_panic() {
        let handle = thread::spawn(|| panic!("oops, something went wrong"));

        match handle.join() {
            Ok(_) => println!("thread finish"),
            Err(_) => println!("thread panic"),
        }

        println!("Application done");
    }

    /*
    BARRIER
    - Barrier merupakan tipe data yang bisa digunakan agar beberapa thread menunggu sebelum melakukan pekerjaannya bersamaan
    - Contoh, kita akan membuat kode program yang menunggu jika 10 thread sudah ada, baru semuanya berjalan, jika belum 10 thread,
    maka program tidak boleh jalan terlebih dahulu
     */

    #[test]
    fn test_barrier() {
        let barrier = Arc::new(Barrier::new(10));
        let mut handles = vec![];

        for i in 0..10 {
            let barrier_clone = Arc::clone(&barrier);
            let handle = thread::spawn(move || {
                println!("Thread {} is waiting", i);
                barrier_clone.wait();
                println!("Thread {} is running", i);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    /*
    ONCE
    - Kadang ada kasus kita membuat variable yang perlu diinisialisasikan datanya diawal cukup sekali saja
    - Namun ingin memastikan bahwa hanya ada satu thread yang bisa memanggil proses inisialisasi datanya
    - Kita bisa menggunakan Once untuk membantu hal ini
    - Once bisa menjaga bahwa hanya ada satu thread saja yang bisa memanggil proses inisialisasi, dan hanya sekali saja dipanggil
     */

    static mut TOTAL_COUNTER: i32 = 0;
    static TOTAL_INIT: Once = Once::new();

    fn get_total() -> i32 {
        unsafe {
            TOTAL_INIT.call_once(|| {
                println!("Initializing total counter");
                TOTAL_COUNTER += 1;
            });
            TOTAL_COUNTER
        }
    }

    #[test]
    fn test_once() {
        let mut handles = vec![];

        for i in 0..10 {
            let handle = thread::spawn(move || {
                let total = get_total();
                println!("Thread {}: {}", i, total);
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    /*
    FUTURE
    - Future adalah representasi dari komputasi asynchronus
    - Future merupakan value yang memungkinkan komputasinya belum selesai. Dengan menggunakan Future,
    memungkinkan thread untuk melanjutkan pekerjaan lainnya, selama menunggu nilainya ada pada Future
    - Future mirip dengan Promise di Javascript, atau mirip pada Future di Java

    POLL
    - Future memiliki satu method bernama poll(), yang digunakan untuk mengambil data di Future
    - Hasil dari poll() method adalah data enum poll
    - Pada enum poll, terdapat dua opsi, Ready jika data sudah ada, dan Pending jika data belum tersedia

    MEMBUAT FUTURE
    - Future merupakan Trait, untuk membuat Future, kita perlu menggunakan method dengan kata kunci async
    - Method dengan kata kunci async, secara otomatis datanya akan mengembalikan tipe data Future

    ASYNC
    - Untuk membuat Future, kita tidak buat secara manual, kita akan menggunakan kata kunci async
    - Function yang menggunakan kata kunci async, maka return value nya adalah Future

    MEMANGGIL KODE ASYNC
    - Kode async tidak bisa dipanggil pada kode non async, oleh karena itu untuk memanggil kode async,
    kita harus menggunakan kode async
    - Sayangnya, secara default, Rust hanya menyediakan kontrak untuk membuat kode async, ketika ingin menjalankan kode async,
    kita perlu menggunakan Runtime/ Executor, dan secara default, Rust tidak menyediakannya
    - Oleh karena itu, kita perlu menggunakan library Runtime/ Executor, untuk menjalankan kode async

    ASYNC TEST
    - Untuk melakukan pengetesan kode Async, kita bisa menggunakan library Tokio
    - Hal ini karena secara default Rust tidak mendukung unit test kode Async
    - Kita bisa menggunakan attribute tokio::test

    AWAIT
    - Secara default, Future merupakan tipe data Lazy, artinya tidak akan dieksekusi jika tidak dijalankan
    - Agar Future dieksekusi, kita bisa menggunakan Await
    - Await hanya bisa digunakan dalam kode Async, karena yang dilakukan Await sebenernya adalah melakukan poll() terhadap Future,
    berbeda dengan join() pada Thread
     */

    async fn get_async_data() -> String {
        thread::sleep(Duration::from_secs(1));
        println!("Fetching async data...");
        return "Async Data".to_string();
    }

    #[tokio::test]
    async fn test_async() {
        let data = get_async_data().await;
        println!("Received async data: {}", data);
    }

    /*
    MASALAH DENGAN THREAD
    - Salah satu permasalahan dengan Thread adalah Thread masih dianggap mahal jika kita menggunakan terlalu banyak
    - Thread akan dijalankan dalam OS (Operating System) Thread, yang artinya ukuran per Thread bisa mencapai 2 - 4 MB
    - Dengan begitu, akan sangat terbatas dengan jumlah memory yang kita gunakan
    - Di bahasa pemrograman seperti Golang atau Kotlin, terdapat fitur Lightweight Thread, seperti Goroutines atau Coroutines
    - Di Rust, fitur ini juga tersedia dan bernama Task

    TOKIO TASK
    - Rust menyediakan kontrak untuk Task, namun implementasinya tetap kita perlu menggunakan Runtime Async yang kita gunakan
    - Kita bisa menggunakan Tokio Task untuk membuat Task, dan cara penggunaannya mirip seperti Thread
    - Yang perlu diperhatikan adalah, saat menggunakan Task, jangan menggunakan fitur Thread seperti Sleep,
    karena itu bisa menghentikan Thread yang digunakan oleh Task

    CONCURRENT
    - Task adalah implementasi dari Concurrent, dimana jika kita menggunakan Thread, Thread tidak bisa berpindah - pindah pekerjaan,
    harus menyelesaikan pekerjaan sampai selesai
    - Sedangkan Task, sebenernya secara internal, Task tetap akan dijalankan dalam Thread, namun Thread yang menjalankan Task,
    bisa berpindah - pindah Task sesuai kebutuhan, misal kita menghentikan Task dengan sleep(), Thread akan menjalankan Task yang lainnya
     */

    async fn get_database_data(wait: u64) -> String {
        println!("{:?} : Fetching database data...", thread::current().id());
        tokio::time::sleep(Duration::from_secs(wait)).await;
        println!("{:?} : Hello from database", thread::current().id());
        return "Hello from database".to_string();
    }

    #[tokio::test]
    async fn test_concurrent() {
        let mut handles = vec![];

        for i in 0..10 {
            let handle = tokio::spawn(get_database_data(i));
            handles.push(handle);
        }

        for handle in handles {
            let data = handle.await.unwrap();
            println!("Received data: {}", data);
        }
    }

    async fn run_concurrent(runtime: Arc<Runtime>) {
        let mut handles = vec![];

        for i in 0..10 {
            let handle = runtime.spawn(get_database_data(i));
            handles.push(handle);
        }

        for handle in handles {
            let data = handle.await.unwrap();
            println!("Received data: {}", data);
        }
    }

    #[test]
    fn test_runtime() {
        let runtime = Arc::new(
            tokio::runtime::Builder::new_multi_thread()
                .worker_threads(10)
                .build()
                .unwrap(),
        );

        runtime.block_on(run_concurrent(Arc::clone(&runtime)));
    }
}
