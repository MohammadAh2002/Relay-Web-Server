use relay_web_server::thread_pool;


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Thread pool size must be greater than zero")]
    fn define_thread_pool_number_zero(){

        thread_pool::ThreadPool::new(0);

    }
    
    #[test]
    fn define_right_number_of_threads(){

        let pool = thread_pool::ThreadPool::new(4);

        assert_eq!(pool.worker_count(), 4);

    }
}