# mako
 
Mako is a safe, secure, performant STS for distributed microservices. An STS is necessary when you want to secure your microservices with JWT's. This is typically a part of a zero trust network environment and is used in conjuction with mTLS. 

There are several different JWT workflows that mako supports and these are outlined in the following sections.

The Manning Press book, ["Microservices Security IN ACTION"],(https://www.manning.com/books/microservices-security-in-action) does an excellent job of providing a detailed overview of the different JWT use cases if you are unfamiliar with them.

# Building

Mako STS uses [gRPC](https://grpc.io/) and requires the [Protocol Buffer compiler](https://github.com/protocolbuffers/protobuf#protocol-compiler-installation) for building.

Please follow the instructions for the [Protocol Compiler](https://github.com/protocolbuffers/protobuf#protocol-compiler-installation) installation, adding the location of protoc to your path prior to building mako.


# STS Workflows

## User Context
A JWT is generated by the STS that includes the user context from the users bearer token.

![Mako STS Shared](images/mako-sts-shared_jwt.png)


## User Context with Unique Audience 
A unique JWT is generated for each target service in this flow. The user context is passed from service to service through tokens issues by the STS and the audience is specific to the target service.

![Mako STS Unique Audience](images/mako-sts-unique_jwt.png)

## Service Identity with Unique Audience
In this flow the service generates a certificate backed token that nests the STS token. This provides a security context that uniquely identifies the calling service and the user/external entity that initiated the workflow in a manner that is very difficult to forge. 


