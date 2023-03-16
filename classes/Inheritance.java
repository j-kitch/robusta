import com.jkitch.robusta.Robusta;

public class Inheritance {

    static {
        Robusta.println(Inheritance.class.getName() + ".<clinit>");
    }

    public static class Animal {

        static {
            Robusta.println(Animal.class.getName() + ".<clinit>");
        }

        protected String name;

        public Animal(String name) {
            this.name = name;
        }

        public String hello() {
            return "Hello, I'm an animal and my name is " + name;
        }
    }

    public static class Dog extends Animal {

        static {
            Robusta.println(Dog.class.getName() + ".<clinit>");
        }

        private String breed;

        public Dog(String name, String breed) {
            super(name);
            this.breed = breed;
        }

        public String hello() {
            return "Woof, I'm a " + breed + ", and my name is " + name;
        }
    }

    public static class Cat extends Animal {

        static {
            Robusta.println(Cat.class.getName() + ".<clinit>");
        }

        public Cat(String name) {
            super(name);
        }

        public String hello() {
            return "I'm a cat, I'm too good for you!";
        }
    }

    public static void main(String[] args) {
        Animal dave = new Dog("Dave", "Whippet");
        Animal frank = new Cat("Frank");
        Animal animal = new Animal("Bird");

        Robusta.println(dave.hello());
        Robusta.println(frank.hello());
        Robusta.println(animal.hello());
    }
}