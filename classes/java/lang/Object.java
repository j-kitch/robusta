package java.lang;

public class Object {

    private native void registerHashCode();
    public Object() {
        registerHashCode();
    }

    public native int hashCode();

    public boolean equals(Object obj) {
        return (this == obj);
    }

    public String toString() {
        return getClass().getName() + "@" + Integer.toHexString(hashCode());
    }

    public final native Class<?> getClass();
}