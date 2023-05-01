/**
 * @file:	async_tcp_daytime_server.cpp
 * @author:	Jacob Xie
 * @date:	2023/05/01 09:15:03 Monday
 * @brief:
 * https://think-async.com/Asio/asio-1.28.0/doc/asio/tutorial/tutdaytime3.html
 **/

#include <asio.hpp>
#include <ctime>
#include <iostream>
#include <string>

using asio::ip::tcp;

std::string make_daytime_string()
{
  using namespace std; // For time_t, time and ctime;
  time_t now = time(0);
  return ctime(&now);
}

// We will use shared_ptr and enable_shared_from_this because we want to keep the
// TcpConnection object alive as long as there is an operation that refers to it.
class TcpConnection : public std::enable_shared_from_this<TcpConnection>
{
public:
  using pointer = std::shared_ptr<TcpConnection>;

  static pointer create(asio::io_context& io_context)
  {
    return pointer(new TcpConnection(io_context));
  }

  tcp::socket& socket()
  {
    return socket_;
  }

  // In the function start(), we call asio::async_write() to serve the data to the client.
  // Note that we are using asio::async_write(), rather than ip::tcp::socket::async_write_some(),
  // to ensure that the entire block of data is sent.
  void start()
  {
    // The data to be sent is stored in the class member message_ as we need to keep the data valid
    // until the asynchronous operation is complete.
    message_ = make_daytime_string();

    // When initiating the asynchronous operation, and if using boost::bind,
    // you must specify only the arguments that match the handler's parameter list.
    // In this program, both of the argument placeholders (asio::placeholders::error
    // and asio::placeholders::bytes_transferred) could potentially have been removed,
    // since they are not being used in handle_write().

    // asio::async_write(
    //     socket_,
    //     asio::buffer(message_),
    //     boost::bind(
    //         &tcp_connection::handle_write,
    //         shared_from_this(),
    //         asio::placeholders::error,
    //         asio::placeholders::bytes_transferred
    //     )
    // );

    // Since we are not using `boost` here, replace them by `std::bind` & `std::placeholders`
    // Moreover, instead of `shared_from_this()`, use `shared_from_this().get()`
    asio::async_write(
        socket_,
        asio::buffer(message_),
        std::bind(
            &TcpConnection::handle_write,
            shared_from_this().get(),
            std::placeholders::_1,
            std::placeholders::_2
        )
    );
  }

  // Any further actions for this client connection are now the responsibility of handle_write().
private:
  TcpConnection(asio::io_context& io_context) : socket_(io_context) {}

  // You may have noticed that the error, and bytes_transferred parameters are not used
  // in the body of the handle_write() function. If parameters are not needed,
  // it is possible to remove them from the function so that it looks like:
  void handle_write(const asio::error_code&, size_t) {}

  tcp::socket socket_;
  std::string message_;
};

//
class TcpServer
{
public:
  // The constructor initialises an acceptor to listen on TCP port 13.
  TcpServer(asio::io_context& io_context) : io_context_(io_context),
                                            acceptor_(io_context, tcp::endpoint(tcp::v4(), 13))
  {
    start_accept();
  }

private:
  // The function start_accept() creates a socket and initiates an asynchronous accept
  // operation to wait for a new connection.
  void start_accept()
  {
    TcpConnection::pointer new_connection = TcpConnection::create(io_context_);

    acceptor_.async_accept(
        new_connection->socket(),
        std::bind(
            &TcpServer::handle_accept,
            this,
            new_connection,
            std::placeholders::_1
        )
    );
  }

  // The function handle_accept() is called when the asynchronous accept operation initiated by
  // start_accept() finishes. It services the client request, and then calls start_accept() to
  // initiate the next accept operation.
  void handle_accept(TcpConnection::pointer new_connection, const asio::error_code& error)
  {
    if (!error)
    {
      new_connection->start();
    }

    start_accept();
  }

  asio::io_context& io_context_;
  tcp::acceptor acceptor_;
};

int main(int argc, char** argv)
{
  try
  {
    // We need to create a server object to accept incoming client connections.
    // The io_context object provides I/O services, such as sockets, that the server object will use.
    asio::io_context io_context;
    TcpServer server(io_context);
    // Run the io_context object so that it will perform asynchronous operations on your behalf.
    io_context.run();
  }
  catch (std::exception& e)
  {
    std::cerr << e.what() << std::endl;
  }

  return 0;
}
