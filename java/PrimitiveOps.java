public class PrimitiveOps {

    public static void main(String[] args) {
        int i = Robusta.parseInt(args[0]);

        intOperations(i);
    }

    private static void intOperations(int i) {
        Robusta.println(i * 54326);
        Robusta.println(i + 4325435);
        Robusta.println(i / 3);
        Robusta.println(i - 54326);
    }
}
