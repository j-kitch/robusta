import com.jkitch.robusta.Robusta;

public class Threads {

    static class SleepAndPrint extends Thread {

        public void run() {
            Thread.sleep(2000);
            Robusta.println("Thread " + Thread.currentThread().getName() + " has slept");
        }
    }

    public static void main(String[] args) {
        Thread first = new SleepAndPrint();
        first.start();
        Robusta.println("Waiting for first: 100ms");
        first.join(100L);
        Robusta.println("Returned from first");
        first.join();

        Thread second = new SleepAndPrint();
        second.start();
        Robusta.println("Waiting for second");
        second.join();
        Robusta.println("Returned from second");
    }
}