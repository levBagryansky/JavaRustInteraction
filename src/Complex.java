public class Complex {
    double re;
    double im;
    Complex(double re, double im){
        this.re = re;
        this.im = im;
    }
    public void Inc(double re, double im){
        this.re += re;
        this.im += im;
    }
    public double Distance(){
        return Math.sqrt(this.re * this.re + this.im * this.im);
    }
}
