public class PrimitiveOps {

    public static void main(String[] args) {
        int i = Robusta.parseInt(args[0]);
        long l = Robusta.parseLong(args[1]);
        float f = Robusta.parseFloat(args[2]);
        double d = Robusta.parseDouble(args[3]);

        intOperations(i);
        longOperations(l);
        floatOperations(f);
        doubleOperations(d);
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
        Robusta.println((byte) i);
        Robusta.println((short) i);
        Robusta.println((char) i);
        Robusta.println((long) i);
        Robusta.println((float) i);
        Robusta.println((double) i);
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
        Robusta.println((byte) l);
        Robusta.println((short) l);
        Robusta.println((char) l);
        Robusta.println((int) l);
        Robusta.println((float) l);
        Robusta.println((double) l);
    }

    private static void floatOperations(float f) {
        Robusta.println(f * 54326);
        Robusta.println(f + 4325435);
        Robusta.println(f / 3);
        Robusta.println(f - 54326);
        Robusta.println(f % 54326);
        Robusta.println(f++);
        Robusta.println(f--);
        Robusta.println(-f);
        Robusta.println((byte) f);
        Robusta.println((short) f);
        Robusta.println((char) f);
        Robusta.println((int) f);
        Robusta.println((long) f);
        Robusta.println((double) f);
    }

    private static void doubleOperations(double d) {
        Robusta.println(d * 54326);
        Robusta.println(d + 4325435);
        Robusta.println(d / 3);
        Robusta.println(d - 54326);
        Robusta.println(d % 54326);
        Robusta.println(d++);
        Robusta.println(d--);
        Robusta.println(-d);
        Robusta.println((byte) d);
        Robusta.println((short) d);
        Robusta.println((char) d);
        Robusta.println((int) d);
        Robusta.println((long) d);
        Robusta.println((float) d);
    }
}
