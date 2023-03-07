import com.jkitch.robusta.Robusta;

public class Exceptions {

    public static void main(String[] args) throws Throwable {
        try {
            throw new RuntimeException("hello world");
        } catch (RuntimeException e) {
            Robusta.println(e.getMessage());
        }

        try {
            throw new Exception("hello world");
        } catch (RuntimeException e) {
            Robusta.println("Caught runtime");
        } catch (Exception e) {
            Robusta.println("Caught exception");
        } finally {
            Robusta.println("Finally did this!");
        }

        throw new Throwable("throwing at the end");
    }
}