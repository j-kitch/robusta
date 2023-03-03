# Issues

## Class Loading

Class loading as defined in the JVM spec is a complex process, split into
a number of distinct steps, where classes are defined by the loader that
loaded them.

We currently have a very simple process of:
- MethodArea registers the loading thread for a class.
- ClassFileLoader loads a class file
- MethodArea loads the runtime representation of the class file.
- MethodArea denotes which classes are initialized `<clinit>`.

I'm slightly uncomfortable with the fact that our `loader` module only
has responsibility for loading class files, not classes themselves, and
where the separation of responsibility between the method area & the class
loader needs to be defined.

I feel like this issue will raise it's head sooner or later.

## Inheritance

We need to define an approach to dealing with inheritance and class loading
properly, at the moment we have assumed no inheritance is required.

## Testing

We have some basic unit testing within modules but this is not particularly
sufficient.
