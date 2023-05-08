public class WaitAndNotify {

    private static class WaitAndPrint extends Thread {

        private final Object lock;
        private final long wait;
        private final String message;

        public WaitAndPrint(Object lock, long wait, String message) {
            this.lock = lock;
            this.wait = wait;
            this.message = message;
        }

        @Override
        public void run() {
            try {
                synchronized (lock) {
                    lock.wait(wait);
                    System.out.println(message);
                }
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }

    private static class WaitIndefinitely extends Thread {

        private final Object lock;

        public WaitIndefinitely(Object lock) {
            this.lock = lock;
        }

        @Override
        public void run() {
            try {
                synchronized (lock) {
                    System.out.println("Waiting for notify");
                    lock.wait();
                    System.out.println("Notified!");
                }
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }

    private static class Notifier extends Thread {

        private final Object lock;

        public Notifier(Object lock) {
            this.lock = lock;
        }

        @Override
        public void run() {
            try {
                synchronized (lock) {
                    lock.notifyAll();
                }
            } catch (Exception e) {
                e.printStackTrace();
            }
        }
    }

    public static void main(String[] args) throws Exception {
        Object lock = new Object();
        Thread first = new WaitAndPrint(lock, 1000, "First Message");
        Thread second = new WaitAndPrint(lock, 1000, "Second Message");

        first.start();
        first.join();
        second.start();
        second.join();

        Thread waiter = new WaitIndefinitely(lock);
        waiter.start();
        Thread notifier = new Notifier(lock);
        notifier.start();
        waiter.join();
    }
}
