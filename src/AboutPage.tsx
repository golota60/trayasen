const AboutPage = () => {
  return (
    <>
      <img src="/carrot.png" alt="A carrot logo" />
      <h1 className="text-4xl mt-2 mb-3">About</h1>
      <p>
        This lovely little app has been created by Szymon Wiszczuk(
        <a target="_blank" href="https://github.com/golota60" rel="noreferrer">
          github
        </a>
        ,
        <a
          target="_blank"
          href="https://twitter.com/SzymonWiszczuk"
          rel="noreferrer"
        >
          twitter
        </a>
        )
      </p>
      <p>
        If something doesn't work for you - please file an issue under{" "}
        <a
          target="_blank"
          href="https://github.com/golota60/idasen-tray/issues/new"
          rel="noreferrer"
        >
          this link
        </a>
        .
      </p>
    </>
  );
};

export default AboutPage;
