class Main {
    // This declares that the static `concat` method will be provided
    // a native library.
    private static native String concat(String word1, String word2);

    private static native double negDistance(Complex comp);

    static {
        // This actually loads the shared object that we'll be creating.
        // The actual location of the .so or .dll may differ based on your
        // platform.
        System.load("/home/tardis3/JavaExpr/src/mylib/target/debug/libmylib.so");
        System.load("/home/tardis3/JavaExpr/src/complexes/target/debug/libcomplexes.so");
        //System.loadLibrary("mylib");
    }

    public static void main(String[] args) {
        String output = Main.concat("Hello", "world");
        //System.out.println(output);
        Complex comp = new Complex(2, 2);
        double anti_dist = negDistance(comp);
        System.out.println("anti_dist = " + anti_dist);
    }
}
