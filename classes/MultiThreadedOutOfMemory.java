public class MultiThreadedOutOfMemory {

    static class OOMThread extends Thread {

        public void run() {
            for (int i = 0; i < 1024; i++) {
                char[] chars = new char[9999999];
            }
        }
    }

    /**
     * Attempt to allocate 190Gb of heap memory, across 10
     * threads, requiring multi-threaded garbage collection!
     */
    public static void main(String[] args) {
        Thread[] threads = new Thread[10];

        for (int i = 0; i < threads.length; i++) {
            threads[i] = new OOMThread();
        }

        for (Thread thread : threads) {
            thread.start();
        }

        for (Thread thread : threads) {
            try {
                thread.join();
            } catch (InterruptedException e) {
                e.printStackTrace();
            }
        }
    }
}