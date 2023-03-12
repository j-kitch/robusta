import com.jkitch.robusta.Robusta;

public class Synchronized {

    public static class Data {
        private String packet;

        // True if receiver should wait
        // False if sender should wait
        private boolean transfer = true;

        public synchronized String receive() {
            while (transfer) {
                try {
                    wait();
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                    Robusta.println("Thread Interrupted");
                }
            }
            transfer = true;

            String returnPacket = packet;
            notifyAll();
            return returnPacket;
        }

        public synchronized void send(String packet) {
            while (!transfer) {
                try {
                    wait();
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                    Robusta.println("Thread Interrupted");
                }
            }
            transfer = false;

            this.packet = packet;
            notifyAll();
        }
    }

    public static class Sender extends Thread {
        private Data data;

        public Sender(Data data) {
            this.data = data;
        }

        public void run() {
            String packets[] = {
                    "First packet",
                    "Second packet",
                    "Third packet",
                    "Fourth packet",
                    "End"
            };

            for (int i = 0; i < packets.length; i++) {
                data.send(packets[i]);

                // Thread.sleep() to mimic heavy server-side processing
                try {
                    Thread.sleep(i * 500);
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                    System.err.println("Thread Interrupted");
                }
            }
        }
    }

    public static class Receiver extends Thread {
        private Data load;

        public Receiver(Data load) {
            this.load = load;
        }

        public void run() {
            int i = 0;
            for(String receivedMessage = load.receive();
                !"End".equals(receivedMessage);
                receivedMessage = load.receive()) {

                Robusta.println(receivedMessage);

                //Thread.sleep() to mimic heavy server-side processing
                try {
                    Thread.sleep(i * 500);
                } catch (InterruptedException e) {
                    Thread.currentThread().interrupt();
                    System.err.println("Thread Interrupted");
                } finally {
                     i++;
                }
            }
        }
    }

    public static void main(String[] args) throws Exception {
        Data data = new Data();
        Thread sender = new Sender(data);
        Thread receiver = new Receiver(data);

        sender.start();
        receiver.start();

        sender.join();
        receiver.join();
    }
}