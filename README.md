# bitpayload
*This is a work in progress project and nothing is working yet. As soon as something worth talking about is working I will do a release & update this readme to refelect this.*

# What is the target of this project?

The target is to write a Rust library that allows serialisation & deserialisation of binary encoded messages. The following things should be supported:

* Fields with different kinds of endianess
* Fields with non byte aligned values and sizes
* Arrays with fixed and dynamic sizes
* Structs of fields
* Some field data interpretations (e.g. enums)

It should be possible to use the library to decode these kind of messages:

* Dynamically: The description of the messages is provided during the runtime of the program (e.g. via a configuration file) and the serialization & deserialization can be done in the same program runtime. This is potentially a bit slower then using generated code to decode the messages, but allows the dynamic loading of new messsage descriptions during runtime.
* Statically: The descitpiton of the messages is provided during compile time and optimized code for serialization & deserialization of these messages is generated as part of the compile time.

And finally there should be the ability to either decode messages completly, or selectivly decode only a few fields.

# Message Payload Description
TODO
