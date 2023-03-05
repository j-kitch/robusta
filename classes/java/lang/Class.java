package java.lang;

import com.jkitch.robusta.Robusta;

public final class Class<T> {

    private final String name;

    private Class(String name) {
        this.name = name;
    }

    public String getName() {
        return name;
    }

    public static Class<?> forName(String className) {
        Robusta.loadClass(className);
        return new Class(className);
    }
}