public class InstanceOf {

    public static void main(String[] args) {
        test(new Object());
        test("hello world");
        test(new InstanceOf());
    }

    private static void test(Object object) {
        Object[] arr = {new Object()};

        System.out.println("Object: " + (object instanceof Object));
        System.out.println("Object Array: " + (object instanceof Object[]));

        System.out.println("String: " + (object instanceof String));
        System.out.println("String Array: " + (object instanceof String[]));

        System.out.println("InstanceOf: " + (object instanceof InstanceOf));
        System.out.println("InstanceOf Array: " + (object instanceof InstanceOf[]));
    }
}
