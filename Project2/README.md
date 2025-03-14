# Project 2: SQL Injections and Web Attacks

This assignment will explore three different web-based attacks: SQL Injection,
Cross-Site Scripting (XSS), and Cross-Site Request Forgery (CSRF).

# Getting up and running: QEMU and Docker

## New options to the QEMU command

You will be using the same QEMU VM that you used for Project 1, but this time we will need to set up a bit more port forwarding, so the command has changed somewhat to the following:

   ```
   qemu-system-x86_64 -accel tcg -m 4G -drive if=virtio,format=qcow2,file=csci368.qcow2 -nic user,model=virtio,hostfwd=tcp:127.0.0.1:41422-:22,hostfwd=tcp:127.0.0.1:41481-:41481,hostfwd=tcp:127.0.0.1:41482-:41482,hostfwd=tcp:127.0.0.1:41483-:41483
   ```

The difference is that we added more `hostfwd=...` (port forwarding) options at the end. This will help make it much easier to work outside of your VM.

## Docker

The tasks will all use docker images that have fully-configured servers that are vulnerable to the various attacks.  While this might sound somewhat complicated (docker inside of a VM), when it's all set up right, you'll be able to do the project through your normal browser, outside the VM altogether!

First, make sure there is no service on your host machine that is already listening on ports 41481, 41482, or 41483. (It is unlikely that there is.) How to check this differs a bit between operating systems, and there are various tools/utilities you can use, but Googling "list listening ports \<your OS\>" should point you in the right direction for your operating system. Common utilities include `lsof` on Mac and Linux, and `netstat` on Windows.

This document will step you through the key commands to get docker running, but for more information, please see [this Docker tutorial](https://gitlab.cs.umd.edu/mmarsh/docker-tutorial).

### Getting and loading docker images

All docker images are available on Moodle in the form of tar files; there's one for each of the three attacks (sqli.tar, xss.tar, and csrf.tar).

Download these to your computer and then copy them into your QEMU VM using `scp` (or a shared directory, if you set that up for Project 1). Assuming the VM is running, then from your host OS (outside the VM), you can simply run the following to copy the .tar file over:

   ```
   scp -P41422 sqli.tar csci368@localhost:
   ```
(This example shows it for the SQL injection docker image, `sqli.tar`; just change the filename to copy the other ones.)

Next, ssh into the QEMU VM and load each docker image individually with the `docker image load` command, which can sometimes take up to a couple minutes in the VM. For instance, to load the SQL injection docker image, run the following from within the QEMU VM:

   ```
   docker image load -i sqli.tar
   ```

In testing, we sometimes found that running shell commands (like this one) from within VS Code could be extremely slow. For that reason, it's *highly* recommended that you run these commands outside of VS Code, via a plain ssh session.

**Please note!** I will have no access to the server containers that you are running locally, so you will need to include *everything* necessary to demonstrate your attacks in the files you submit. Pay close attention to the submission instructions in each section.

## More project compatibility notes

For the Java code you will be writing, please ensure that it is compliant with **Java 1.8**.

For the task `.txt` files you submit, make sure that your editor does not insert "smart" quotes; our auto-grader will be using good old-fashioned single quotes (ASCII character 0x27) and double quotes (ASCII character 0x22).

For all tasks, pay close attention to the formatting specified. There's a difference between single and double quotes, and the automated grading will be very unhappy if you use single quotes where you should have used double quotes. Not following the formatting properly is *the major reason* for missing points on task 2. In particular, the initial lines for the first 2 tasks represent the fields the auto-grader will enter into the relevant HTML forms, so please include **all** of the needed fields **exactly** as you would *enter* them manually. For tasks 3 and 5, the files you provide will be used verbatim and in their entirety, so please do not include additional information. For task 5, you may include HTML comments, but for task 3, if you feel the need to include additional information, please place it in a separate file.


# Task 1: SQL Injection Attack

## Setup
Your goal in this task is to find and exploit a SQL-Injection vulnerability.

To run the docker image:

1. Start the server by running the following command from within your QEMU VM:

   ```
   docker run -d -p 41481:80 --name sql_server sqli
   ```

(It may take a moment for the server to start up -- if step 2 or 3 below doesn't work at first, give it a minute and try again.)

2. To test that the web server is running inside the VM, you can run `curl localhost:41481`; if the web server is running, you will see HTML printed to stdout.

3. You can access the server in your own browser -- outside the VM altogether! -- at

    http://localhost:41481/

You will not have to modify any of the files inside of the docker container or QEMU VM; all of your work for this task will be taking place strictly by interacting with the vulnerable webpage through your browser. However, it will be very helpful for you to examine some of the files in the docker image.

If you want to examine files in the docker container, you have several options, all of which you have to run from within your QEMU VM:

 1. `docker exec -ti sql_server bash`
 2. `docker exec sql_server cat <filename>`
 3. `docker cp sql_server:<filename> <local filename>`

The first option gives you a shell on the running container. The commands available to you via this option are somewhat limited, but enough to navigate the file system and look at the contents of files. The second will simply dump the contents of the file (replace `<filename>` with the full path to the file) to STDOUT. The third will copy a file from the container to a local destination, and behaves very similarly to the normal `cp` command.

For this task, the server you are attacking is called Collabtive, which is a web-based project management system. It has several user accounts configured. To see all the users' account information, first log in as the admin using the password below; other users' account information can be obtained from the post on the front page ("Users' Account Information"; then click on the "Description" drop-down toggle near the top of the page).

   ```
   username: admin 
   password: admin
   ```

## SQL Injection on UPDATE Statements

In this task, you need to make an unauthorized modification to the database. Your goal is to modify another user's profile using SQL injections. In Collabtive, if users want to update their profiles, they can go to "My account", click the "Edit" link, and then fill out a form to update the profile information. After the user sends the update request to the server, an `UPDATE` statement will be constructed in `include/class.user.php`. (There are several `UPDATE` statements in this file; inspect the file to determine which one is used when editing a user's profile.)

The objective of this statement is to modify the current user's profile information in the users table. There is a SQL injection vulnerability in this SQL statement. Please find the vulnerability, and then use it to change Bob's profile without knowing their password. For example, if you are logged in as Alice, your goal is to use the vulnerability to modify Bob's profile information, including Bob's password. After the attack, you should be able to log into Bob's account with this new password.

Note that passwords are not stored in their raw form in the database. If you examine the `include/class.user.php` file, you will see how passwords are stored and checked.

### My Notes
- They store passwords as SHA-1 hash values instead of their plaintext. 
- The `edit` function is the thing you're looking for. You can find it by searching `/function edit` in Vi.
- There's something called `$mylog` which is an event log for the current session maybe?
    - the `class mylog` is stored in `class.mylog.php` under the same directory.
    - It seems that the `add` function takes four arguments: `$name`, `$type`, `$action` and `$project`. 
    - `$action` can be in the range [1,7]. The most important one is that 2 denotes "edited".
    - These all get added to a table called `log`.
- This is also used to store transactions.
- Almost everything in here is wrapped in `class user`
- There is another function `function editpass` which I think is the one that is sent when we try to edit a user's password.
    - This function takes four arguments: `$id`, `$oldpass`, `$newpass`, `$repeatpass`.
    - The `$pass` variables are all wrapped in a function called `mysql_real_escape_string`.
	- Presumably this sanitizes them?
	- It actually just prepends backslashes to the following characters: \x00, \n, \r, \, ', " and \x1a. 
    - Ohhh okay wer're getting somewhere.
	- There's a variable `$chk` which performs an unsanitized SQL query and is the thing that checks
	whether you are allowed to change a user's password.
	- It has the structure `$chk = mysql_query("SELECT ID, name FROM user WHERE ID = $id AND pass = '$oldpass'")`
	- First they set `$oldpass = sha1($oldpass)`.
	- They do the same for `$newpass`.
	- If `$newpass != $oldpass`, the function returns before the SQL query.
	- `$id` is an integer that presumably corresponds to the user we're looking for.
	    - We need some way to find Bob's `$id`.
	- If it's in the same order as they appear in the user data and they're zero-indexed, bob
	would have $id = 5.
- Turns out I was wrong. The function we're looking for is `function edit`.
    - In this function, almost everything is sanitized with an exception: the state name.
	- However, the state name removed the `=` character.  
    - The telephone name is also unsanitized.
	- However, the telephone removes the `-` character.
    - This means that we have to do all sorts of assignment in the telephone and then comment out all the undesirable SQL stuff in the state name.
    - Attempt #1:
	- Phone: ```
	    x', pass='1abedcd9967cc42ea624432d356a5f0bce7ae3a9
	```
	- State: ``
	    x' WHERE name LIKE 'bob'" -- 
	``

### Submission

Your task is to change Bob's password by modifying a different user's profile. Please submit a file called `task1.txt` with the following format:

 * line 1: Bob's new password after your injection, wrapped in double quotes.

 * line 2: User whose profile you are modifying in order to carry out the attack (*not* Bob or Admin), without quotes.

 * lines 3-n:
     One line for each profile parameter you changed, like so:

     ```
     "param"="new value"
     ```

     Please note that both the parameter name and new value should be wrapped in double quotes, so that we can clearly see any spaces you might have added. Also, the parameter name should be exactly as it appears in the HTML source code, *not* what you see on the webpage. You will have to examine the HTML or network traffic for these. (Hint: If your parameters begin with capital letters or dollar signs, you have the wrong parameter names.) The new value should be *exactly* what you type into the corresponding field of the form, including the injection.  If you don't do this, the automated grading will mark this task as failing. *All* parameters that you need to set must be included here for the automated grading to succeed.

 * line n+1: A blank line 

 * lines n+2 to end: The remaining lines should contain a short write up explaining the steps you used to create a working SQL injection attack that updates Bob's password to a new value. This is especially important if you mis-format the parameter lines, since it's the most likely way we'll be able to figure out what you actually should have entered during manual regrading, if needed.

For example:

    "newpasswd"
    alice
    "foo"="bar"
    ...
    "baz"="blah"
    

    Other explanatory stuff.

When you upload your completed submission, include task1.txt.


# Tasks 2 & 3: XSS Attacks

Your goal in these tasks is to find ways to exploit Cross-Site Scripting vulnerabilities and demonstrate the damage that can be achieved by the attacks.

To run the docker image:

1. (The first time you run the image, if you haven't already loaded the image by running ```docker image load -i sqli.tar``` inside the VM, do that first.)

2. Inside the VM, start the server by running

   ```
   docker run -d -p 41482:80 --name xss_server xss
   ```
   
3. You can access the server in your browser at

    http://localhost:41482/

For these tasks, the server you are attacking is called Elgg, which is a social networking application. It has several user accounts configured, with the following credentials:

    USER     USERNAME  PASSWORD
    =======  ========  ===========
    Admin    admin     seedelgg
    Alice    alice     seedalice
    Boby     boby      seedboby
    Charlie  charlie   seedcharlie
    Samy     samy      seedsamy

## Warmup - No submission Necessary 

The objective of this task is to embed a JavaScript program in your Elgg profile, such that when another user views your profile, the JavaScript program will be executed and an alert window will be displayed. The following JavaScript program will display an alert window:
 	
      <script>alert('XSS')</script> 

If you embed the above JavaScript code in your profile (e.g. in the brief description field), then any user who views your profile will see the alert window.
  
Our next objective is to embed a JavaScript program in your Elgg profile, such that when another user views your profile, the user's cookies will be displayed in the alert window. This can be done by adding some additional code to the JavaScript program in the previous example:
  
      <script>alert(document.cookie);</script> 

## Task 2: Stealing Cookies from the Victim's Machine 

In the warmup task, the malicious JavaScript code written by the attacker can print out the user's cookies, but only the user can see the cookies, not the attacker. In this task, the attacker wants the JavaScript code to send the cookies to themselves. To achieve this, the malicious JavaScript code needs to send an HTTP request to the attacker, with the cookies appended to the request.
 
We can do this by having the malicious JavaScript insert an `<img>` tag with `src` attribute set to the attacker's machine. When the JavaScript inserts the `img` tag, the browser tries to load the image from the URL in the `src` field; this results in an HTTP GET request sent to the attacker's machine. Your JavaScript code should send the cookies to the port 41485 of the attacker's machine, where the attacker has a TCP server listening to the same port. The server can print out whatever it receives. The TCP server program is included as `echoserv.py`, which is a python script, and can be run outside the VM.
 
### Submission

Please submit a file called `task2.txt`. The grading will be done as follows:

1. We will run the echo server on localhost on port 41485.
2. We will edit the brief description field acting as the user Alice using the message contents as specified by your `task2.txt`. We will use the *entire* contents of this file, so if you need to provide additional details, please do so in a separate file.
1. Then, when Boby opens this message, Boby's cookie should be printed by the echo server.

If for whatever reason you needed to test on a port other than 41485, don't forget to change it back to 41485 once it's working! That's the port we'll be using when grading.

## Task 3: Session Hijacking using the Stolen Cookies   

After stealing the victim's cookies, the attacker can do whatever the victim can do to the Elgg web server, including adding and deleting friends on behalf of the victim, deleting the victim's posts, etc. Essentially, the attacker has hijacked the victim's session. In this task, we will launch this session hijacking attack, and write a program to remove one of the victim's friends.
 
To remove one of the victim's friends, we should first find out how a legitimate user removes a friend in Elgg. More specifically, we need to figure out what HTTP headers are sent to the server when a user removes a friend. Here is a screenshot: ![screenshot](RemoveFriendRequestScreenshot.png)

For this project, it may be helpful to observe HTTP requests and responses directly -- a quick Google should tell you how to see requests and responses in the browser of your choice. (For instance, in Chrome and Firefox, simply right-click on a webpage and choose "Inspect", then the "Network" tab. Now, if you make a request to remove a friend, you should see a resource named `remove?friend...` that you can click on to see headers and data. From the contents, you can identify all the parameters in the request.)
 
Once we have understood what the HTTP request for removing friends looks like, we can write a Java program to send out the same HTTP request. The Elgg server cannot distinguish whether the request is sent out by the user's browser or by the attacker's Java program. As long as we set all the parameters correctly, and the session cookie is attached, the server will accept and process the HTTP request. To simplify your task, we provide you with a starter Java program, `HTTPSimpleForge.java`, that does the following:

1. Open a connection to web server.  
2. Set the necessary HTTP header information.  
3. Send the request to web server.  
4. Get the response from web server. 
 
**Note 1:** Elgg uses two parameters `__elgg_ts` and `__elgg_token`. Make sure that you set these parameters correctly for your attack to succeed.  Also, please note down the correct guid of the friend who needs to be added to the friend list.  You need to use that guid in the program code for the attack to succeed.

**Note 2:** You can compile a Java program into bytecode for **Java 1.8** by running

    javac --release=8 HTTPSimpleForge.java

on the console. You can then run the bytecode by running

    java HTTPSimpleForge  
 
### Submission

Please submit a file called `HTTPSimpleForge.java`.

### Grading

Your java file will be compiled into byte code and executed (from outside the VM). Before running your compiled code, we will login as Alice and add Boby as a friend to Alice. Then, we will try to remove Boby as Alice's friend by running your compiled code. Make sure you include the correct guid and parameters in the code so that the above scenario executes properly!
 
**Input:** Your java program should read from an input file called `HTTPSimpleForge.txt`. This filename should be hard-coded into your program; it *will not* be passed on the command line. The first line of the input file contains the `__elgg_ts` token (absolute value), the second line contains the `__elgg_token` (absolute value) and the third line would contain the Cookie HTTP header value:

    Elgg=<<cookie value>>

As an example:

    1402467511
    80923e114f5d6c5606b7efaa389213b3
    Elgg=7pgvml3vh04m9k99qj5r7ceho4

You can create a text file locally for testing out your code; however, you do not need to submit this file as part of your submission. We will use our own `HTTPSimpleForge.txt` file for checking.

### My Notes
- Adding a friend produces the following GET request:
```
http://localhost:41482/action/friends/add?friend=40&__elgg_ts=1741283347&__elgg_token=b678b7df613f4943fa24f5e32fcb58f3
```
- This was what happened when I added Boby as a friend from Alice's account.
- Removing a friend produces the following GET request:
```
http://localhost:41482/action/friends/remove?friend=40&__elgg_ts=1741283351&__elgg_token=27f3598550554619312600a42ebc20f1
```
- This was similarly what happened when I tried to remove Boby as a friend from Alice's account.
- I can't find the rhyme or reason to the `elgg_token` or the `elgg_ts` variables.
    - The `elgg_ts` variable always seems to go up--presumably by a number of seconds--but I don't
    know how I'd hard-code a variable like this.
    - Meanwhile, the `elgg_token` variable seems entirely random. Probably a SHA-1 hash.
- I think that the `guid = 40` for Boby.


# CSRF Attack

Your goal in this task is to find ways to exploit a Cross-Site Request Forgery vulnerability.

Steps for running docker image:

1. Start the server by running the following from within the QEMU VM:

   ```
   docker run -d -p 41483:80 -v "$(pwd):/var/www/CSRF/Attacker" --name csrf_server csrf
   ```

2. You can access the server in your browser (outside the VM) at

    http://localhost:41483/

This is running a slightly different version of Elgg, but has the same users and credentials as in the XSS tasks.
 
## Task 4: CSRF Attack using GET Request 
 
In this task, we need two people in the Elgg social network: Alice and Boby. Alice wants to become a friend to Boby, but Boby refuses to add Alice to his Elgg friend list. Alice decides to use the CSRF attack to achieve her goal. She
sends Boby a URL (via an email or a posting in Elgg); Boby, curious about it, clicks on the URL. Pretend that you are Alice, and think about how you can construct the contents of the web page, so as soon as Boby visits the web page, Alice is added to the friend list of Boby (assuming Boby has an active session with Elgg).

To add a friend to the victim, we need to identify the Add Friend HTTP request, which is a GET request. In this task, you are not allowed to write JavaScript code to launch the CSRF attack. Your job is to make the attack successful as soon as Boby visits the web page, without even clicking on the page (hint: one good solution involves using the `img` tag, which automatically triggers an `HTTP GET` request).

Whenever the victim user visits the crafted web page in the malicious site, the web browser automatically issues an `HTTP GET` request for the URL contained in the `img` tag. Because the web browser automatically attaches the session cookie to the request, the trusted site cannot distinguish the malicious request from the genuine request and ends up processing the request, compromising the victim user's session integrity.

Observe the request structure for adding a new friend and then use this to forge a new request to the application. When the victim user visits the malicious web page, a malicious request for adding a friend should be injected into the victim's active session with Elgg.
 
### Submission

You are required to submit a file named `task4.html`. When a victim user named Boby is logged in, and visits the attacker website `localhost:41483/task4.html` in another tab, Alice should be added as a friend to Boby's Friend List.
 
To test this, you will need to place the `task4.html` file under the directory `/var/www/CSRF/elgg/` in the docker container. You can do this by running the following from within the QEMU VM (assuming `task4.html` has already been copied into the VM):
  
    docker cp task5.html csrf_server:/var/www/CSRF/elgg

**Tip:** Your browser may not refresh on its own. You might need to press the reload/refresh button to reload the page, to see if Alice is added as a friend to Boby's account.

**Remember:** I cannot access your docker containers directly! You will need to include `task4.html` in your submission.

# Submission checklist

When you're finished, you should have one file per task:
* task1.txt - SQL injection attack
* task2.txt - first XSS attack
* HTTPSimpleForge.Java - second XSS attack
* task4.txt - CSRF attack
