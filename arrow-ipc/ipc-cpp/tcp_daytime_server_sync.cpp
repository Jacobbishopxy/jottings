/**
 * @file:	sync_tcp_daytime_server.cpp
 * @author:	Jacob Xie
 * @date:	2023/04/30 23:25:39 Sunday
 * @brief:
 * https://think-async.com/Asio/asio-1.28.0/doc/asio/tutorial/tutdaytime2.html
 **/

#include <asio.hpp>
#include <ctime>
#include <iostream>
#include <string>

using asio::ip::tcp;

// We define the function make_daytime_string() to create the string to be sent back to the client.
// This function will be reused in all of our daytime server applications.
std::string make_daytime_string()
{
  using namespace std; // For time_t, time and ctime;
  time_t now = time(0);
  return ctime(&now);
}

int main(int argc, char** argv)
{
  try
  {
    asio::io_context io_context;

    // A ip::tcp::acceptor object needs to be created to listen for new connections.
    // It is initialised to listen on TCP port 13, for IP version 4.
    tcp::acceptor acceptor(io_context, tcp::endpoint(tcp::v4(), 13));

    // This is an iterative server, which means that it will handle one connection at a time.
    // Create a socket that will represent the connection to the client, and then wait for a connection.
    for (;;)
    {
      tcp::socket socket(io_context);
      acceptor.accept(socket);

      // A client is accessing our service.
      // Determine the current time and transfer this information to the client.
      std::string msg = make_daytime_string();

      asio::error_code ignored_error;
      asio::write(socket, asio::buffer(msg), ignored_error);
    }
  }
  // Finally, handle any exceptions.
  catch (std::exception& e)
  {
    std::cerr << e.what() << std::endl;
  }

  return 0;
}
