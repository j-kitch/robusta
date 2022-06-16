package java.lang;

public class Object {

    public boolean equals(Object object) {
        return this == object;
    }

    public native int hashCode();

    public native Class<?> getClass();
}
