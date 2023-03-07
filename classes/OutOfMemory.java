public class OutOfMemory {

    public static void main(String[] args) {
        int heapSize = 1280 * 1024 * 1024;
        int arrayLength = 1280 * 1024;
        for (int i = 0; i < 1024; i++) {
            char[] chars = new char[9999999];
        }
    }
}