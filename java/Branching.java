public class Branching {

    public static void main(String[] args) {
        int i1 = Integer.parseInt(args[0]);
        int i2 = Integer.parseInt(args[1]);

        intBranches(i1, i2);
    }

    private static void intBranches(int i1, int i2) {
        if (i1 == i2) {
            System.out.println("i1 and i2 are equal");
        }

        if (i1 != i2) {
            System.out.println("i1 and i2 are not equal");
        }

        if (i1 < i2) {
            System.out.println("i1 is less than i2");
        }

        if (i1 <= i2) {
            System.out.println("i1 is less than or equal to i2");
        }

        if (i1 > i2) {
            System.out.println("i1 is greater than i2");
        }

        if (i1 >= i2) {
            System.out.println("i1 is greater than or equal to i2");
        }
    }
}
