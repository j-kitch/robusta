public class Waiting {

    public static void main(String[] args) throws Exception {
        Object obj = new Object();
        synchronized (obj) {
            obj.wait(2000);
            System.out.println("Hello World");

            obj.wait(2000);
            System.out.println("Hello World");
        }
    }
}