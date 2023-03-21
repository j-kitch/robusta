public class Throws {

    public static void main(String[] args) throws Exception {
        try {
            System.out.println("Starting main");
            if (args.length > 0 && args[0].equals("main")) {
                throw new Exception("throwing in main");
            }
            foo(args);
        } catch (IllegalStateException e) {
            System.out.println("Caught illegal state exception in main: " + e.getMessage());
            throw e;
        } catch (RuntimeException e) {
            System.out.println("Caught runtime exception in main: " + e.getMessage());
            throw e;
        } catch (Exception e) {
            System.out.println("Caught exception in main: " + e.getMessage());
            throw e;
        } finally {
            System.out.println("Finally main");
        }
        System.out.println("Finishing main");
    }

    private static void foo(String[] args) {
        try {
            System.out.println("Starting foo");
            if (args.length > 0 && args[0].equals("foo")) {
                throw new IllegalStateException("throwing in foo");
            }
            bar(args);
        } catch (IllegalStateException e) {
            System.out.println("Caught illegal state exception in foo: " + e.getMessage());
            throw e;
        } catch (RuntimeException e) {
            System.out.println("Caught runtime exception in foo: " + e.getMessage());
            throw e;
        } finally {
            System.out.println("Finally foo");
        }
        System.out.println("Finishing foo");
    }

    private static void bar(String[] args) {
        try {
            System.out.println("Starting bar");
            if (args.length > 0 && args[0].equals("bar")) {
                throw new RuntimeException("throwing in bar");
            }
        } catch (RuntimeException e) {
            System.out.println("Caught runtime exception in bar: " + e.getMessage());
            throw e;
        } finally {
            System.out.println("Finally bar");
        }
        System.out.println("Finishing bar");
    }
}