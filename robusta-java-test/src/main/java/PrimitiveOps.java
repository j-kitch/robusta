public class PrimitiveOps {

    public static void main(String[] args) {
        int i = Integer.parseInt(args[0]);
        long l = Long.parseLong(args[1]);
        float f = Float.parseFloat(args[2]);
        double d = Double.parseDouble(args[3]);

        intOperations(i);
        longOperations(l);
        floatOperations(f);
        doubleOperations(d);
    }

    private static void intOperations(int i) {
        System.out.println(i * 54326);
        System.out.println(i + 4325435);
        System.out.println(i / 3);
        System.out.println(i - 54326);
        System.out.println(i % 54326);
        System.out.println(i++);
        System.out.println(i--);
        System.out.println(i & 54326);
        System.out.println(i | 54326);
        System.out.println(i ^ 54326);
        System.out.println(~i);
        System.out.println(i << 5);
        System.out.println(i >> 5);
        System.out.println(i >>> 5);
        System.out.println(-i);
        System.out.println((byte) i);
        System.out.println((short) i);
        System.out.println((char) i);
        System.out.println((long) i);
        System.out.println((float) i);
        System.out.println((double) i);
    }

    private static void longOperations(long l) {
        System.out.println(l * 54326);
        System.out.println(l + 4325435);
        System.out.println(l / 3);
        System.out.println(l - 54326);
        System.out.println(l % 54326);
        System.out.println(l++);
        System.out.println(l--);
        System.out.println(l & 54326);
        System.out.println(l | 54326);
        System.out.println(l ^ 54326);
        System.out.println(~l);
        System.out.println(l << 5);
        System.out.println(l >> 5);
        System.out.println(l >>> 5);
        System.out.println(-l);
        System.out.println((byte) l);
        System.out.println((short) l);
        System.out.println((char) l);
        System.out.println((int) l);
        System.out.println((float) l);
        System.out.println((double) l);
    }

    private static void floatOperations(float f) {
        System.out.println(f * 54326);
        System.out.println(f + 4325435);
        System.out.println(f / 3);
        System.out.println(f - 54326);
        System.out.println(f % 54326);
        System.out.println(f++);
        System.out.println(f--);
        System.out.println(-f);
        System.out.println((byte) f);
        System.out.println((short) f);
        System.out.println((char) f);
        System.out.println((int) f);
        System.out.println((long) f);
        System.out.println((double) f);
    }

    private static void doubleOperations(double d) {
        System.out.println(d * 54326);
        System.out.println(d + 4325435);
        System.out.println(d / 3);
        System.out.println(d - 54326);
        System.out.println(d % 54326);
        System.out.println(d++);
        System.out.println(d--);
        System.out.println(-d);
        System.out.println((byte) d);
        System.out.println((short) d);
        System.out.println((char) d);
        System.out.println((int) d);
        System.out.println((long) d);
        System.out.println((float) d);
    }
}
