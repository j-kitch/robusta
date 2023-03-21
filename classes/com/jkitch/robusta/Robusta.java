package com.jkitch.robusta;

public class Robusta {

    public static String internString(char[] chars) {
        String string = new String(chars);
        return string.intern();
    }

    public static native Class<?> loadClass(String name);

    public static native void println(String string);

    public static native void printerr(String string);

    public static native void println(int i);

    public static native void exit(int code);

    public static void throwThrowable(Throwable throwable) throws Throwable {
        throw throwable;
    }

    public static void printStackTrace(Throwable throwable) {
        printerr(throwable.toString());
        for (StackTraceElement element : throwable.getStackTrace()) {
            printerr("\t" + element.getFileName() + ":" + element.getLineNumber() + ": " + element.getClassName() + "." + element.getMethodName());
        }
    }
}