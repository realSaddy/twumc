import React from "react";
import jwt_decode from "jwt-decode";
import { BrowserRouter as Router, Switch, Route, Link } from "react-router-dom";

import Login from "./Login";

export default function App() {
  return (
    <Router>
      {authenticated()[0] ? (
        <div>
          <nav>
            <ul>
              <li>
                <Link to="/">Home</Link>
              </li>
              <li>
                <Link to="/test">Test</Link>
              </li>
            </ul>
          </nav>
        </div>
      ) : null}
      <Switch>
        <AuthedRoute path="/test" component={Test} />
        <AuthedRoute path="/" component={Index} />
      </Switch>
    </Router>
  );
}

/**
 * Returns whether the user is authenticated & with their token if they are
 */
function authenticated() {
  try {
    const t = localStorage.getItem("auth");
    const token = jwt_decode(t);
    console.log(token);
    if (token.exp < Date.now()) {
      throw new Error();
    }
    return [true, token];
  } catch (err) {
    if (localStorage.getItem("auth") !== null) localStorage.removeItem("auth");
    return [false, null];
  }
}

function AuthedRoute({ component, ...rest }) {
  return (
    <Route
      {...rest}
      render={(_props) => {
        const [authed, token] = authenticated();
        if (authed) {
          return <Component token={token} {...rest} />;
        } else {
          return <Login />;
        }
      }}
    />
  );
}

function Test() {
  return <h2>test</h2>;
}

function Index() {
  return <h1>Index</h1>;
}
