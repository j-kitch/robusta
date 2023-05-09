public class MathsInstructions {

    public static void main(String[] args) {
        System.out.println(ints());
    }

    private static int ints() {
        // bipush
        int i = 10;
        System.out.println(i);

        // sipush
        i = 32766;
        System.out.println(i);

        // iinc
        i += 40;
        System.out.println(i);

        // ldc
        int j = 54235245;
        System.out.println(j);

        // iadd
        i += j;
        System.out.println(i);

        // iadd overflow
        i += Integer.MAX_VALUE;
        System.out.println(i);

        // isub
        i -= 54235;
        System.out.println(i);

        // isub overflow
        i -= Integer.MAX_VALUE;
        System.out.println(i);

        // imul
        i *= 5423;
        System.out.println(i);

        // imul overflow
        i *= Integer.MAX_VALUE;
        System.out.println(i);

        // idiv
        i /= 5432;
        System.out.println(i);

        // irem
        i %= 432;
        System.out.println(i);

        // iand
        i &= 5432;
        System.out.println(i);

        // ior
        i |= 542352;
        System.out.println(i);

        // ixor
        i ^= 4312;
        System.out.println(i);

        // ishl
        i <<= 5;
        System.out.println(i);

        // ishr
        i >>= 5;
        System.out.println(i);

        // iushr positive;
        i >>>= 3;
        System.out.println(i);

        // iushr negative;
        i = -5423;
        i >>>= 12;
        System.out.println(i);

        // ineg positive
        i = -i;
        System.out.println(i);

        // ineg negative
        i = -i;
        System.out.println(i);

        // ineg 0
        i = 0;
        i = -i;
        System.out.println(i);

        // iconst_m1
        i = -1;
        System.out.println(i);

        // iconst_0
        i = 0;
        System.out.println(i);

        // iconst_1
        i = 1;
        System.out.println(i);

        // iconst_2
        i = 2;
        System.out.println(i);

        // iconst_3
        i = 3;
        System.out.println(i);

        // iconst_4
        i = 4;
        System.out.println(i);

        // iconst_5
        i = 5;
        System.out.println(i);

        // get an "unknown" number
        i = i > 4 ? 5642652 : -543522;

        // i2b
        byte b = (byte) i;
        System.out.println(b);

        // i2c
        char c = (char) i;
        System.out.println(c);

        // i2s
        short s = (short) i;
        System.out.println(s);

        // i2f
        float f = (float) i;
        System.out.println(f);

        // i2d
        double d = (double) i;
        System.out.println(d);

        // i2l
        long l = i;
        System.out.println(l);

        return i;
    }
}
