/**
 * @file:	sync_tcp_daytime_client.cpp
 * @author:	Jacob Xie
 * @date:	2023/04/30 23:25:58 Sunday
 * @brief:
 * https://think-async.com/Asio/asio-1.28.0/doc/asio/tutorial/tutdaytime1.html
 **/

#include <array>
#include <asio.hpp>
#include <iostream>

using asio::ip::tcp;

int main(int argc, char** argv)
{

  // The purpose of this application is to access a daytime service,
  // so we need the user to specify the server.

  // if (argc != 2)
  // {
  //   std::cerr << "Usage: client <host>" << std::endl;
  //   return 1;
  // }

  // Instead of using the code from example, `addr` and `port` is manually specified
  const char* addr = "127.0.0.1";
  const char* port = "13";

  try
  {
    // All programs that use asio need to have at least one I/O execution context,
    // such as an io_context object.
    asio::io_context io_context;
    // We need to turn the server name that was specified as a parameter to the application, into a TCP endpoint.
    // To do this we use an ip::tcp::resolver object.
    tcp::resolver resolver(io_context);

    // A resolver takes a host name and service name and turns them into a list of endpoints.
    // We perform a resolve call using the name of the server, specified in argv[1],
    // and the name of the service, in this case "daytime".
    // The list of endpoints is returned using an object of type ip::tcp::resolver::results_type.
    // This object is a range, with begin() and end() member functions that may be used for iterating over the results.
    // tcp::resolver::results_type endpoints = resolver.resolve(argv[1], "daytime");

    // Instead of using the code from the example, use specified values
    tcp::resolver::results_type endpoints = resolver.resolve(addr, port);

    // Now we create and connect the socket.
    // The list of endpoints obtained above may contain both IPv4 and IPv6 endpoints,
    // so we need to try each of them until we find one that works.
    // This keeps the client program independent of a specific IP version.
    // The asio::connect() function does this for us automatically.
    tcp::socket socket(io_context);
    asio::connect(socket, endpoints);

    // The connection is open.
    // All we need to do now is read the response from the daytime service.
    for (;;)
    {
      // We use a boost::array to hold the received data.
      // The asio::buffer() function automatically determines the size of the array to help prevent buffer overruns.
      // Instead of a boost::array, we could have used a char [] or std::vector.
      std::array<char, 128> buf;
      asio::error_code err;

      size_t len = socket.read_some(asio::buffer(buf), err);

      // When the server closes the connection, the ip::tcp::socket::read_some() function will exit with
      // the asio::error::eof error, which is how we know to exit the loop.
      if (err == asio::error::eof)
      {
        std::cout << "eof" << std::endl;
        break;
      }
      else if (err)
        throw asio::system_error(err);

      std::cout.write(buf.data(), len);
    }
  }
  // Finally, handle any exceptions that may have been thrown.
  catch (std::exception& e)
  {
    std::cerr << e.what() << std::endl;
  }

  return 0;
}
