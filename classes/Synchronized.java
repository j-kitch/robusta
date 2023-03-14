import com.jkitch.robusta.Robusta;

public class Synchronized {

    public static synchronized void staticSync() {
        try {
            Thread.sleep(1000);
            Robusta.println("staticSync " + Thread.currentThread().getName());
        } catch (Exception e) {

        }
    }

    public static synchronized void sync(Object o) {
        synchronized (o) {
            try {
                Thread.sleep(1000);
                Robusta.println("sync " + Thread.currentThread().getName());
            } catch (Exception e) {

            }
        }
    }

    static class StaticSync extends Thread {
        public void run() {
            staticSync();
        }
    }

    static class ObjectSync extends Thread {
        Object lock;

        public void run() {
            synchronized (lock) {
                sync(lock);
            }
        }
    }

    public static void main(String[] args) {
        Thread[] threads = new Thread[5];
        for (int i = 0; i < 5; i++) {
            threads[i] = new StaticSync();
        }

        for (Thread thread : threads) {
            thread.start();
        }

        for (Thread thread : threads) {
            try {
                thread.join();
            } catch (Exception e) {

            }
        }

        Object globalLock = new Object();
        for (int i = 0; i < 5; i++) {
            ObjectSync thread = new ObjectSync();
            thread.lock = globalLock;
            threads[i] = thread;
        }

        for (Thread thread : threads) {
            thread.start();
        }

        for (Thread thread : threads) {
            try {
                thread.join();
            } catch (Exception e) {

            }
        }
    }
}