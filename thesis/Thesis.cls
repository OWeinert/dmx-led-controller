%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
% Thesis LaTeX Template - THESIS CLASS FILE
%
% This template has been downloaded from:
% http://www.latextemplates.com
%
% This class file defines the structure and design of the template. 
%
% There is one part of this file that needs to be filled out - the variables
% dictating the document particulars such as the author name, university
% name, etc. You will find these in the commented "DOCUMENT VARIABLES"
% section below.
%
% The other two easily-editable sections are the margin sizes and abstract.
% These have both been commented for easy editing. Advanced LaTeX
% users will have no trouble editing the rest of the document to their liking.
%
% Original header:
%% This is file `Thesis.cls', based on 'ECSthesis.cls', by Steve R. Gunn
%% generated with the docstrip utility.
%%
%% Created by Steve R. Gunn, modified by Sunil Patel: www.sunilpatel.co.uk
%% Further modified by www.latextemplates.com
%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

\NeedsTeXFormat{LaTeX2e}[1996/12/01]
\ProvidesClass{Thesis}
[2007/22/02 v1.0
LaTeX document class]
\def\baseclass{book}
\DeclareOption*{\PassOptionsToClass{\CurrentOption}{\baseclass}}
\def\@checkoptions#1#2{
	\edef\@curroptions{\@ptionlist{\@currname.\@currext}}
	\@tempswafalse
	\@tfor\@this:=#2\do{
		\@expandtwoargs\in@{,\@this,}{,\@curroptions,}
		\ifin@ \@tempswatrue \@break@tfor \fi}
	\let\@this\@empty
	\if@tempswa \else \PassOptionsToClass{#1}{\baseclass}\fi
}
\@checkoptions{11pt}{{10pt}{11pt}{12pt}}
\PassOptionsToClass{a4paper}{\baseclass}
\ProcessOptions\relax
\LoadClass{\baseclass}
\newcommand\bhrule{\typeout{------------------------------------------------------------------------------}}

\newcommand\Declaration[1]{
	\btypeout{Declaration of Authorship}
	\addtotoc{Declaration of Authorship}
	\thispagestyle{plain}
	\null\vfil
	%\vskip 60\p@
	\begin{center}{\huge\bf Declaration of Authorship\par}\end{center}
	%\vskip 60\p@
	{\normalsize #1}
	\vfil\vfil\null
	%\cleardoublepage
}

\newcommand\btypeout[1]{\bhrule\typeout{\space #1}\bhrule}
\def\today{\ifcase\month\or
	January\or February\or March\or April\or May\or June\or
	July\or August\or September\or October\or November\or December\fi
	\space \number\year}
\usepackage{setspace}
\onehalfspacing
\setlength{\parindent}{0pt}
\setlength{\parskip}{2.0ex plus0.5ex minus0.2ex}
\usepackage{vmargin}

\RequirePackage[utf8]{inputenc} % Allows the use of international characters (e.g. Umlauts)


% ---------------
% --- Margins ---
% ---------------
\setmarginsrb
{ 1.0in}  % left margin
{ 0.6in}  % top margin
{ 1.0in}  % right margin
{ 0.8in}  % bottom margin
{  20pt}  % head height
{0.25in}  % head sep
{   9pt}  % foot height
{ 0.3in}  % foot sep

% ---------------
\raggedbottom
\setlength{\topskip}{1\topskip \@plus 5\p@}
\doublehyphendemerits=10000       % No consecutive line hyphens.
\brokenpenalty=10000              % No broken words across columns/pages.
\widowpenalty=9999                % Almost no widows at bottom of page.
\clubpenalty=9999                 % Almost no orphans at top of page.
\interfootnotelinepenalty=9999    % Almost never break footnotes.
\usepackage{fancyhdr}
\newcommand{\pageNumberSize}{\Large}
\fancypagestyle{plain}{%
	\fancyhf{} % clear all header and footer fields 
	\fancyfoot[C]{\pageNumberSize\thepage} % except the center
	\renewcommand{\headrulewidth}{0pt}
	\renewcommand{\footrulewidth}{0pt}}
\lhead[\rm\thepage]{\fancyplain{}{\sl{\rightmark}}}
\rhead[\fancyplain{}{\sl{\leftmark}}]{\rm\pageNumberSize\thepage}
\chead{}\lfoot{}\rfoot{}\cfoot{}
\pagestyle{fancy}
\renewcommand{\chaptermark}[1]{\btypeout{\thechapter\space #1}\markboth{\@chapapp\ \thechapter\ #1}{\@chapapp\ \thechapter\ #1}}
\renewcommand{\sectionmark}[1]{}
\renewcommand{\subsectionmark}[1]{}
\def\cleardoublepage{\clearpage\if@twoside \ifodd\c@page\else
	\hbox{}
	\thispagestyle{empty}
	\newpage
	\if@twocolumn\hbox{}\newpage\fi\fi\fi}
\usepackage{amsmath,amsfonts,amssymb,amscd,amsthm,xspace}
\theoremstyle{plain}
\newtheorem{example}{Example}[chapter]
\newtheorem{theorem}{Theorem}[chapter]
\newtheorem{corollary}[theorem]{Corollary}
\newtheorem{lemma}[theorem]{Lemma}
\newtheorem{proposition}[theorem]{Proposition}
\newtheorem{axiom}[theorem]{Axiom}
\theoremstyle{definition}
\newtheorem{definition}[theorem]{Definition}
\theoremstyle{remark}
\newtheorem{remark}[theorem]{Remark}
\usepackage[centerlast,small,sc]{caption}
\setlength{\captionmargin}{20pt}
\newcommand{\fref}[1]{Figure~\ref{#1}}
\newcommand{\tref}[1]{Table~\ref{#1}}
\newcommand{\eref}[1]{Equation~\ref{#1}}
\newcommand{\cref}[1]{Chapter~\ref{#1}}
\newcommand{\sref}[1]{Section~\ref{#1}}
\newcommand{\aref}[1]{Appendix~\ref{#1}}
\renewcommand{\topfraction}{0.85}
\renewcommand{\bottomfraction}{.85}
\renewcommand{\textfraction}{0.1}
\renewcommand{\dbltopfraction}{.85}
\renewcommand{\floatpagefraction}{0.75}
\renewcommand{\dblfloatpagefraction}{.75}
\setcounter{topnumber}{9}
\setcounter{bottomnumber}{9}
\setcounter{totalnumber}{20}
\setcounter{dbltopnumber}{9}
\usepackage{graphicx}
\usepackage{epstopdf}
\usepackage{booktabs}
\usepackage{rotating}
\usepackage{listings}
\usepackage{lstpatch}
\usepackage{titlesec}
\usepackage{tocloft}
\usepackage{tikz}
\lstset{captionpos=b,
	frame=tb,
	basicstyle=\scriptsize\ttfamily,
	showstringspaces=false,
	keepspaces=true}
\lstdefinestyle{matlab} {
	language=Matlab,
	keywordstyle=\color{blue},
	commentstyle=\color[rgb]{0.13,0.55,0.13}\em,
	stringstyle=\color[rgb]{0.7,0,0} }
\usepackage[pdfpagemode={UseOutlines},bookmarks=true,bookmarksopen=true,
bookmarksopenlevel=0,bookmarksnumbered=true,hypertexnames=false,
colorlinks,linkcolor={black},citecolor={black},urlcolor={black},
pdfstartview={FitV},unicode,breaklinks=true]{hyperref}

\pdfstringdefDisableCommands{
	\let\\\space
}


% --------------------------
% ----- Shadow Images  -----
% --------------------------

% some parameters for customization
\def\shadowshift{0pt,0pt}
\def\shadowradius{15pt}

\colorlet{innercolor}{black!30}
\colorlet{outercolor}{gray!00}

% this draws a shadow under a rectangle node
\newcommand\drawshadow[1]{
	\begin{pgfonlayer}{shadow}
		\shade[outercolor,inner color=innercolor,outer color=outercolor] ($(#1.south west)+(\shadowshift)+(\shadowradius/2,\shadowradius/2)$) circle (\shadowradius);
		\shade[outercolor,inner color=innercolor,outer color=outercolor] ($(#1.north west)+(\shadowshift)+(\shadowradius/2,-\shadowradius/2)$) circle (\shadowradius);
		\shade[outercolor,inner color=innercolor,outer color=outercolor] ($(#1.south east)+(\shadowshift)+(-\shadowradius/2,\shadowradius/2)$) circle (\shadowradius);
		\shade[outercolor,inner color=innercolor,outer color=outercolor] ($(#1.north east)+(\shadowshift)+(-\shadowradius/2,-\shadowradius/2)$) circle (\shadowradius);
		\shade[top color=innercolor,bottom color=outercolor] ($(#1.south west)+(\shadowshift)+(\shadowradius/2,-\shadowradius/2)$) rectangle ($(#1.south east)+(\shadowshift)+(-\shadowradius/2,\shadowradius/2)$);
		\shade[left color=innercolor,right color=outercolor] ($(#1.south east)+(\shadowshift)+(-\shadowradius/2,\shadowradius/2)$) rectangle ($(#1.north east)+(\shadowshift)+(\shadowradius/2,-\shadowradius/2)$);
		\shade[bottom color=innercolor,top color=outercolor] ($(#1.north west)+(\shadowshift)+(\shadowradius/2,-\shadowradius/2)$) rectangle ($(#1.north east)+(\shadowshift)+(-\shadowradius/2,\shadowradius/2)$);
		\shade[outercolor,right color=innercolor,left color=outercolor] ($(#1.south west)+(\shadowshift)+(-\shadowradius/2,\shadowradius/2)$) rectangle ($(#1.north west)+(\shadowshift)+(\shadowradius/2,-\shadowradius/2)$);
		\filldraw ($(#1.south west)+(\shadowshift)+(\shadowradius/2,\shadowradius/2)$) rectangle ($(#1.north east)+(\shadowshift)-(\shadowradius/2,\shadowradius/2)$);
	\end{pgfonlayer}
}

% create a shadow layer, so that we don't need to worry about overdrawing other things
\pgfdeclarelayer{shadow} 
\pgfsetlayers{shadow,main}


\newcommand\shadowimage[2][]{%
	\begin{tikzpicture}
		\node[anchor=south west,inner sep=0] (image) at (0,0) {\includegraphics[#1]{#2}};
		\drawshadow{image}
\end{tikzpicture}}


% --------------------------
% --- Document variables ---
% --------------------------
\newcommand*{\thesistitle}[1]{\def\ttitle{#1}}
\thesistitle{Veranschaulichung digitaler Steuerprotokolle in der Lichttechnik}
\def\authorname{Klaus, Conrad Yosemite}
\def\submissionDate{25. April 2023}
   
% ------------------------------
% --- Custom chapter/section headder --- 
% ------------------------------
\titleformat{\chapter}%
{\bfseries\Huge}
{\thechapter.}
{10pt}
{}

\titleformat{\section}%
{\bfseries\LARGE}
{\thesection.}
{5pt}
{}


% The default title page, this is not used in this www.latextemplates.com version            
\renewcommand\maketitle{
	\btypeout{Title Page}
	\hypersetup{pdftitle={\@title}}
	\hypersetup{pdfsubject=\subjectname}
	\hypersetup{pdfauthor=\authornames}
	\hypersetup{pdfkeywords=\keywordnames}
	\thispagestyle{empty}
	\begin{titlepage}
		\let\footnotesize\small
		\let\footnoterule\relax
		\let \footnote \thanks
		\setcounter{footnote}{0}
		\null\vfil
		\vskip 60\p@
		\begin{center}
			\setlength{\parskip}{0pt}
			{\large\textbf{\UNIVNAME}\par}
			\vfill
			{\huge \bf \@title \par}
			\vfill
			{\LARGE by \par}
			\smallskip
			{\LARGE \authornames \par}
			\vfill
			{\large A thesis submitted in partial fulfillment for the \par}
			{\large degree of \degreename \par}
			\bigskip
			\bigskip
			{\large in the \par}
			{\large \facname \par}
			{\large \deptname \par} 
			\bigskip
			\bigskip
			\bigskip
			{\Large \@date \par}
			\bigskip
		\end{center}
		\par
		\@thanks
		\vfil\null
	\end{titlepage}
	\setcounter{footnote}{0}%
	\global\let\thanks\relax
	\global\let\maketitle\relax
	\global\let\@thanks\@empty
	\global\let\@author\@empty
	\global\let\@date\@empty
	\global\let\@title\@empty
	\global\let\title\relax
	\global\let\author\relax
	\global\let\date\relax
	\global\let\and\relax
	\cleardoublepage
}
%----------------------------------------------------------------------------------------
%	ABSTRACT PAGE DESIGN
%----------------------------------------------------------------------------------------
\newenvironment{abstract}
{
	\btypeout{Abstract Page}
	\thispagestyle{empty}
	\null\vfil
	\begin{center}
		\setlength{\parskip}{0pt}
%		{\normalsize \UNIVNAME \par} % University name in capitals
		\bigskip
		{\huge{\textit{Abstract}} \par}
		\bigskip
		{\normalsize \facname \par} % Faculty name
		{\normalsize \deptname \par} % Department name
		\bigskip
		{\normalsize \degreename\par} % Degree name
		\bigskip
		{\normalsize\bf \@title \par} % Thesis title
		\medskip
		{\normalsize by \authornames \par} % Author name
		\bigskip
	\end{center}
}
%----------------------------------------------------------------------------------------
{
	\vfil\vfil\vfil\null
	\cleardoublepage
}
\addtocounter{secnumdepth}{1}
\setcounter{tocdepth}{6}
\newcounter{dummy}
\newcommand\addtotoc[1]{
	\phantomsection
	\refstepcounter{dummy}
	\addcontentsline{toc}{chapter}{#1}}
\renewcommand{\contentsname}{Inhaltsverzeichnis}
\renewcommand\bibname{Literaturverzeichnis}
\renewcommand{\listfigurename}{Abbildungsverzeichnis}
\renewcommand{\listtablename}{Tabellenverzeichnis}

% custom table of content spacing
\setlength\cftparskip{-7pt}
\setlength\cftbeforechapskip{5pt}
\renewcommand\tableofcontents{
	\btypeout{Table of Contents}
	\begin{spacing}{1}{
			\setlength{\parskip}{1pt}
			\if@twocolumn
			\@restonecoltrue\onecolumn
			\else
			\@restonecolfalse
			\fi
			\chapter*{\contentsname
				\@mkboth{
					\MakeUppercase\contentsname}{\MakeUppercase\contentsname}}
			\@starttoc{toc}
			\if@restonecol\twocolumn\fi
			\cleardoublepage
	}\end{spacing}
}
\renewcommand\listoffigures{
	\btypeout{List of Figures}
	\addtotoc{List of Figures}
	\begin{spacing}{1}{
			\setlength{\parskip}{1pt}
			\if@twocolumn
			\@restonecoltrue\onecolumn
			\else
			\@restonecolfalse
			\fi
			\chapter*{\listfigurename
				\@mkboth{\MakeUppercase\listfigurename}
				{\MakeUppercase\listfigurename}}
			\@starttoc{lof}
			\if@restonecol\twocolumn\fi
			\cleardoublepage
	}\end{spacing}
}
\renewcommand\listoftables{
	\btypeout{List of Tables}
	\addtotoc{List of Tables}
	\begin{spacing}{1}{
			\setlength{\parskip}{1pt}
			\if@twocolumn
			\@restonecoltrue\onecolumn
			\else
			\@restonecolfalse
			\fi
			\chapter*{\listtablename
				\@mkboth{
					\MakeUppercase\listtablename}{\MakeUppercase\listtablename}}
			\@starttoc{lot}
			\if@restonecol\twocolumn\fi
			\cleardoublepage
	}\end{spacing}
}
\newcommand\listsymbolname{Abbreviations}
\usepackage{longtable}
\newcommand\listofsymbols[2]{
	\btypeout{\listsymbolname}
	\addtotoc{\listsymbolname}
	\chapter*{\listsymbolname
		\@mkboth{
			\MakeUppercase\listsymbolname}{\MakeUppercase\listsymbolname}}
	\begin{longtable}[c]{#1}#2\end{longtable}\par
	\cleardoublepage
}
\newcommand\listconstants{Physical Constants}
\usepackage{longtable}
\newcommand\listofconstants[2]{
	\btypeout{\listconstants}
	\addtotoc{\listconstants}
	\chapter*{\listconstants
		\@mkboth{
			\MakeUppercase\listconstants}{\MakeUppercase\listconstants}}
	\begin{longtable}[c]{#1}#2\end{longtable}\par
	\cleardoublepage
}
\newcommand\listnomenclature{Symbols}
\usepackage{longtable}
\newcommand\listofnomenclature[2]{
	\btypeout{\listnomenclature}
	\addtotoc{\listnomenclature}
	\chapter*{\listnomenclature
		\@mkboth{
			\MakeUppercase\listnomenclature}{\MakeUppercase\listnomenclature}}
	\begin{longtable}[c]{#1}#2\end{longtable}\par
	\cleardoublepage
}
\newcommand\acknowledgements[1]{
	\btypeout{Acknowledgements}
	\addtotoc{Acknowledgements}
	\thispagestyle{plain}
	\begin{center}{\huge{\textit{Acknowledgements}} \par}\end{center}
	{\normalsize #1}
	\vfil\vfil\null
	
}
\newcommand\dedicatory[1]{
	\btypeout{Dedicatory}
	\thispagestyle{plain}
	\null\vfil
	\vskip 60\p@
	\begin{center}{\Large \sl #1}\end{center}
	\vfil\null
	\cleardoublepage
}
\renewcommand\backmatter{
	\if@openright
	\cleardoublepage
	\else
	\clearpage
	\fi
	\addtotoc{\bibname}
	\btypeout{\bibname}
	\@mainmatterfalse}
\endinput



