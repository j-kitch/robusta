public class StringConcat {

    public static void main(String[] args) {
        String arg = args[0];
        int greetingIndex = Integer.parseInt(args[1]);

        String[] greetings = {"welcome", "wilkommen", "bienvenido", "salut"};
        String greeting = greetings[greetingIndex];

        String message = arg + ", " + greeting;
        System.out.println(message);
    }
}
