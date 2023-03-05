package com.jkitch.robusta;

public class Robusta {

    public static String internString(char[] chars) {
        String string = new String(chars);
        return string.intern();
    }

    public static native Class<?> loadClass(String name);

    public static native void println(String string);

    public static native void println(int i);
}