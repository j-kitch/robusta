public class PrimitiveOps {

    public static void main(String[] args) {
        int i = Robusta.parseInt(args[0]);
        long l = Robusta.parseLong(args[1]);

        intOperations(i);
        longOperations(l);
    }

    private static void intOperations(int i) {
        Robusta.println(i * 54326);
        Robusta.println(i + 4325435);
        Robusta.println(i / 3);
        Robusta.println(i - 54326);
        Robusta.println(i % 54326);
        Robusta.println(i++);
        Robusta.println(i--);
        Robusta.println(i & 54326);
        Robusta.println(i | 54326);
        Robusta.println(i ^ 54326);
        Robusta.println(~i);
        Robusta.println(i << 5);
        Robusta.println(i >> 5);
        Robusta.println(i >>> 5);
        Robusta.println(-i);
    }

    private static void longOperations(long l) {
        Robusta.println(l * 54326);
        Robusta.println(l + 4325435);
        Robusta.println(l / 3);
        Robusta.println(l - 54326);
        Robusta.println(l % 54326);
        Robusta.println(l++);
        Robusta.println(l--);
        Robusta.println(l & 54326);
        Robusta.println(l | 54326);
        Robusta.println(l ^ 54326);
        Robusta.println(~l);
        Robusta.println(l << 5);
        Robusta.println(l >> 5);
        Robusta.println(l >>> 5);
        Robusta.println(-l);
    }
}
