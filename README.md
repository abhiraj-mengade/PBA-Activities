# Multi Pad

## Solution 

Group 5
Members:
```
1. Abhiraj Mengade
2. Rohit Sarpotdar
3. Oliver
4. Kishan
```

The challenge is to solve for the Key of this Multi Pad Scheme.
We have added a rust as well as a python implementation for breaking this scheme.

The trick is to notice that `XOR` with space char and a letter will give the capital char of the letter.

The key that appears on completing the Challenge is:
```
bitcoins implementation of a peer-to-peer version of electronic cash would allow online payments to be sent directly from one party to another without going through a financial institution
```

While the plaintexts are (Deciphered):
```
The Times 03/Jan/2009 Chancellor on brink of second bailout for banks.
Governments are good at cutting off the heads of a centrally controlled networks like Napster, but pure P2P networks like Gnutella and Tor seem to be holding their own.
Bitcoin is great as a form of digital money, but its scripting language is too weak for any kind of serious advanced applications to be built on top.
In order to have a decentralized database, you need to have security. In order to have security, you need to have incentives.
As society becomes more and more complex, cheating will in many ways become progressively easier and easier to do and harder to police or even understand.
I began to realize new possibilities opening up between the fields of ICT and game theory, and the inevitable social change to which this would lead.
Cryptocurrencies allowed non-custodial exchange, without users having to sign up or create accounts.
Not your keys, Not your coins.
```

Drawbacks of Multi Pad:
1. Repetion of Key causes the key to be easily deciphered.
2. The key should be random and should not be repeated.
