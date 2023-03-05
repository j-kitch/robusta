import com.jkitch.robusta.Robusta;

public class HashCode {

    public static void main(String[] args) {
        Robusta.println(new Object().hashCode());
        Robusta.println(new Object().hashCode());
        Robusta.println(new Object().hashCode());
        Robusta.println(new StringBuilder().hashCode());
    }
}