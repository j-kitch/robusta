public class Equals {

    public static void main(String[] args) {
        Object objectA = new Object();
        Object objectB = new Object();

        String stringA = "hello world";
        String stringB = args[0];

        // Object.equals(Object)
        System.out.println("objectA == objectA: " + objectA.equals(objectA));
        System.out.println("objectA == objectB: " + objectA.equals(objectB));
        System.out.println("objectB == objectA: " + objectB.equals(objectA));
        System.out.println("objectB == objectB: " + objectB.equals(objectB));

        // Object.equals(String)
        System.out.println("objectA == stringA: " + objectA.equals(stringA));
        System.out.println("objectA == stringB: " + objectA.equals(stringB));
        System.out.println("objectB == stringA: " + objectB.equals(stringA));
        System.out.println("objectB == stringB: " + objectB.equals(stringB));

        // String.equals(Object)
        System.out.println("stringA == objectA: " + stringA.equals(objectA));
        System.out.println("stringA == objectB: " + stringA.equals(objectB));
        System.out.println("stringB == objectA: " + stringB.equals(objectA));
        System.out.println("stringB == objectB: " + stringB.equals(objectB));

        // String.equals(String)
        System.out.println("stringA == stringA: " + stringA.equals(stringA));
        System.out.println("stringA == stringB: " + stringA.equals(stringB));
        System.out.println("stringB == stringA: " + stringB.equals(stringA));
        System.out.println("stringB == stringB: " + stringB.equals(stringB));

        // String.equals(String copy)
        System.out.println("stringA == stringA: " + stringA.equals(new String(stringA)));
        System.out.println("stringA == stringB: " + stringA.equals(new String(stringB)));
        System.out.println("stringB == stringA: " + stringB.equals(new String(stringA)));
        System.out.println("stringB == stringB: " + stringB.equals(new String(stringB)));
    }
}
