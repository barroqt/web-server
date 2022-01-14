## Development steps

#### 1. Building a single-threaded web server
* Set up tcp/ip listener
* Read request
* Write a response
    * return HTML
    * Validate request
    * selective response
* Refactoring

#### 2. Turning the server into a multithreaded server
* implement thread to simulate a request
* improve throughtput with a thread pool
* create space to store the threads
* Create worker that sends code from the thread pool to a thread
    * Worker struct that holds an id and JoinHandle()
    * Thread pool holds vector of Worker instances
    * Worker::new takes id and returns an instance that holds an id and a thread with empty closure
    * ThreadPool::new generates workers with an id and stores it in a vector
* Send requests to threads via channels
    * ThreadPool creates a channel and hold on to the sending side
    * Each worker will hold on the receiving side
    * new Job struct will hold the closures we want to send down the channel
    * execute method will send the job it wants to execute down the sending side
    * In its thread, the Worker will loop over its receiving side and execute every closure it receives

#### 3. Shutdown and cleanup
* Implement Drop trait on ThreadPool
* Signal thread to stop listening for jobs

## To do:
* Add more documentation to ThreadPool and its public methods.
* Add tests of the library’s functionality.
* Change calls to unwrap to more robust error handling.
* Use ThreadPool to perform some task other than serving web requests.
* Find a thread pool crate on crates.io and implement a similar web server using the crate instead. Then compare its API and robustness to the thread pool we implemented.

