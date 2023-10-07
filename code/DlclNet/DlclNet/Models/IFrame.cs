using DlclRpc;

namespace DlclNet.Models;

public interface IFrame
{
    public IEnumerable<Pixel> Pixels { get; set; }
}