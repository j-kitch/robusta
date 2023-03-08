package java.lang;

public class StackTraceElement {

    private String declaringClass;
    private String methodName;
    private String fileName;

    /**
     * -2 represents a native method.
     */
    private int lineNumber;

    public StackTraceElement(String declaringClass, String methodName, String fileName, int lineNumber) {
        this.declaringClass = declaringClass;
        this.methodName = methodName;
        this.fileName = fileName;
        this.lineNumber = lineNumber;
    }

    public String getClassName() {
        return declaringClass;
    }

    public String getMethodName() {
        return methodName;
    }

    public String getFileName() {
        return fileName;
    }

    public int getLineNumber() {
        return lineNumber;
    }

    public boolean isNativeMethod() {
        return lineNumber == -2;
    }

    public String toString() {
        return declaringClass + "." + methodName + "(" +
                (isNativeMethod() ? "Native Method)" :
                        (fileName != null && lineNumber >= 0 ?
                                fileName + ":" + lineNumber + ")" :
                                (fileName != null ?  ""+fileName+")" : "Unknown Source)")));
    }
}