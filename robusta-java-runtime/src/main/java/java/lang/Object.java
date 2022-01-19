package java.lang;

public class Object {

    public native int hashCode();

    public final native Class<?> getClass();

    public String toString() {
        return getClass().getName() + "@" + Integer.toHexString(hashCode());
    }
}
